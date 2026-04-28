use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::io::{self, Read, Seek, SeekFrom};
use std::num::NonZeroUsize;
use std::str::FromStr;

use tracing::warn;

use crate::error::{EmptyMetadataError, MetadataParseError};
use crate::parse::{ParseResult, parse_delimited_string, parse_value_if_valid};

/// Reads icy metadata contained within a stream.
///
/// Seeking within the stream is supported with the following limitations:
///
/// - [`SeekFrom::End`](std::io::SeekFrom::End) is not supported since seeking from the end of a
///   stream conceptually doesn't make sense.
/// - Seeking backwards is limited by the size of the metadata cache. Since the metadata values have
///   dynamic sizes, we need to know the size of the previous metadata value to seek past it. In
///   order to prevent unbounded memory growth, we cap the number of previous metadata sizes we keep
///   track of. You can change this limit using [`Self::metadata_cache_size`]. In practice, most
///   metadata is 0-sized except for at the start of each track. We use rudimentary compression so
///   consecutive metadata entries of the same size don't take up additional slots in the array.
///   This means you shouldn't exceed the default value of `128` unless you're going really far
///   back.
pub struct IcyMetadataReader<T> {
    inner: T,
    icy_metadata_interval: Option<usize>,
    next_metadata: usize,
    metadata_size_queue: MetadataSizeQueue,
    current_pos: u64,
    on_metadata_read: Box<dyn Fn(Result<IcyMetadata, MetadataParseError>) + Send + Sync>,
}

impl<T> Debug for IcyMetadataReader<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IcyMetadataReader")
            .field("inner", &"<inner>")
            .field("icy_metadata_interval", &self.icy_metadata_interval)
            .field("next_metadata", &self.next_metadata)
            .field("metadata_size_queue", &self.metadata_size_queue)
            .field("current_pos", &self.current_pos)
            .field("on_metadata_read", &"<on_metadata_read>")
            .finish()
    }
}

impl<T> IcyMetadataReader<T> {
    /// Creates a new `IcyMetadataReader`.
    /// `icy_metadata_interval` is required in order to figure out the location of the metadata
    /// blocks. If `icy_metadata_interval` is `None`, it will treat the stream as though the
    /// metadata is absent. You can retrieve the value from
    /// [`IcyHeaders::metadata_interval`](crate::IcyHeaders::metadata_interval) or by extracting the
    /// value from the headers manually.
    pub fn new<F>(
        inner: T,
        icy_metadata_interval: Option<NonZeroUsize>,
        on_metadata_read: F,
    ) -> Self
    where
        F: Fn(Result<IcyMetadata, MetadataParseError>) + Send + Sync + 'static,
    {
        let icy_metadata_interval = icy_metadata_interval.map(|i| i.get());
        Self {
            inner,
            icy_metadata_interval,
            on_metadata_read: Box::new(on_metadata_read),
            next_metadata: icy_metadata_interval.unwrap_or(0),
            metadata_size_queue: MetadataSizeQueue {
                inner: VecDeque::new(),
                cache_size: 128,
            },

            current_pos: 0,
        }
    }
}

impl<T> IcyMetadataReader<T> {
    /// Set the size of the metadata cache.
    pub fn metadata_cache_size(mut self, size: usize) -> Self {
        self.metadata_size_queue.cache_size = size;
        self
    }
}

// The metadata length block must be multiplied by 16 to get the total metadata length
// info taken from here https://gist.github.com/niko/2a1d7b2d109ebe7f7ca2f860c3505ef0
const ICY_METADATA_MULTIPLIER: usize = 16;

impl<T> IcyMetadataReader<T>
where
    T: Read,
{
    fn parse_metadata_from_stream(&mut self, buf: &mut [u8], metaint: usize) -> io::Result<usize> {
        let to_fill = buf.len();
        let mut total_written = 0;
        while total_written < to_fill {
            let prev_written = total_written;
            self.parse_next_metadata(buf, metaint, &mut total_written)?;
            // No additional data written, we're at the end of the stream
            if total_written == prev_written {
                break;
            }
        }
        self.current_pos += total_written as u64;
        Ok(total_written)
    }

    fn parse_next_metadata(
        &mut self,
        buf: &mut [u8],
        metaint: usize,
        total_written: &mut usize,
    ) -> io::Result<()> {
        let to_fill = buf.len();

        if self.next_metadata > 0 {
            // Read data before next metadata, capped to the caller's buffer length
            // so that streams whose metaint exceeds buf.len() don't panic with an
            // out-of-bounds slice (observed on SomaFM's higher-bitrate channels).
            let to_read = self.next_metadata.min(to_fill);
            let written = self.inner.read(&mut buf[..to_read])?;
            if written == 0 {
                return Ok(());
            }
            *total_written += written;
            self.next_metadata -= written;
            if self.next_metadata > 0 {
                // Pre-metadata audio not yet fully drained (buf too small or short
                // read). Defer the metadata block to the next call.
                return Ok(());
            }
        }

        self.read_metadata()?;
        self.next_metadata = metaint;
        let start = *total_written;

        // make sure we don't exceed the buffer length
        let end = (start + self.next_metadata).min(to_fill);
        let written = self.inner.read(&mut buf[start..end])?;
        *total_written += written;
        self.next_metadata = metaint - written;
        Ok(())
    }

    fn update_metadata_size(&mut self) -> io::Result<()> {
        let mut metadata_length_buf = [0u8; 1];
        self.inner.read_exact(&mut metadata_length_buf)?;

        let metadata_length = metadata_length_buf[0] as usize * ICY_METADATA_MULTIPLIER;

        self.metadata_size_queue.push(metadata_length);
        Ok(())
    }

    fn read_metadata(&mut self) -> io::Result<()> {
        self.update_metadata_size()?;
        if let Some(last_size) = self.metadata_size_queue.peek() {
            if last_size > 0 {
                let mut metadata_buf = vec![0u8; last_size];
                self.inner.read_exact(&mut metadata_buf)?;

                let callback_val = String::from_utf8(metadata_buf)
                    .map_err(MetadataParseError::InvalidUtf8)
                    .and_then(|metadata_str| {
                        let metadata_str = metadata_str.trim_end_matches(char::from(0));
                        metadata_str
                            .parse::<IcyMetadata>()
                            .map_err(MetadataParseError::Empty)
                    });
                (self.on_metadata_read)(callback_val);
            }
        }
        Ok(())
    }
}

impl<T> Read for IcyMetadataReader<T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Default to normal read behavior if metaint is not set
        let Some(metaint) = self.icy_metadata_interval else {
            return self.inner.read(buf);
        };

        if buf.len() > self.next_metadata {
            self.parse_metadata_from_stream(buf, metaint)
        } else {
            let read = self.inner.read(buf)?;
            self.next_metadata -= read;
            self.current_pos += read as u64;
            Ok(read)
        }
    }
}

impl<T> Seek for IcyMetadataReader<T>
where
    T: Read + Seek,
{
    fn seek(&mut self, seek_from: io::SeekFrom) -> io::Result<u64> {
        // Default to normal behavior if metaint is not set
        let Some(metaint) = self.icy_metadata_interval else {
            return self.inner.seek(seek_from);
        };

        let (requested_change, requested_pos) = match seek_from {
            SeekFrom::Start(pos) => (pos as i64 - self.current_pos as i64, pos as i64),
            SeekFrom::Current(pos) => (pos, self.current_pos as i64 + pos),
            SeekFrom::End(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "seek from end not supported",
                ));
            }
        };

        let mut current_absolute_pos = self.inner.stream_position()? as i64;
        let mut seek_progress = 0i64;

        if requested_change < 0 {
            let mut last_metadata_offset = (metaint - self.next_metadata) as i64;
            let mut last_metadata_end_pos = current_absolute_pos - last_metadata_offset;

            // Keep seeking back through the previous metadata entries until we reach a point where
            // the next seek point is after the next metadata
            while current_absolute_pos + requested_change - seek_progress < last_metadata_end_pos
                && last_metadata_end_pos > 0
            {
                let Some(last_metadata_size) = self.metadata_size_queue.pop() else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Attempting to seek beyond metadata length cache. You may need to call \
                         IcyMetadataReader::metadata_cache_size to increase the cache size.",
                    ));
                };
                // +1 for the byte that holds the metadata length
                let metadata_region_size = last_metadata_size as i64 + 1;
                let seek_to = (last_metadata_end_pos - metadata_region_size) as u64;
                // Seek before the metadata entry
                current_absolute_pos = self.inner.seek(SeekFrom::Start(seek_to))? as i64;
                seek_progress -= last_metadata_offset;

                last_metadata_offset = metaint as i64;
                last_metadata_end_pos -= metadata_region_size + metaint as i64;
            }
        } else {
            // Keep seeking forward through each metadata entry until we reach a point where
            // the next seek point is before the next metadata
            while requested_change - seek_progress >= self.next_metadata as i64 {
                self.inner
                    .seek(SeekFrom::Current(self.next_metadata as i64))?;
                seek_progress += self.next_metadata as i64;
                // Read the metadata and continue
                self.read_metadata()?;
            }
        }
        self.inner
            .seek(SeekFrom::Current(requested_change - seek_progress))?;
        self.next_metadata = metaint - ((requested_pos as usize) % metaint);
        self.current_pos = requested_pos as u64;
        Ok(self.current_pos)
    }
}

/// Metadata contained within a stream
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IcyMetadata {
    stream_title: Option<String>,
    stream_url: Option<String>,
    custom: HashMap<String, String>,
}

impl IcyMetadata {
    /// The title of the currently playing track.
    /// Maps to the `StreamTitle` metadata value.
    pub fn stream_title(&self) -> Option<&str> {
        self.stream_title.as_deref()
    }

    /// This could be an album art URL, an image URL for the stream itself, or some other
    /// information. Maps to the `StreamUrl` metadata value.
    pub fn stream_url(&self) -> Option<&str> {
        self.stream_url.as_deref()
    }

    /// Any additional fields found in the metadata.
    pub fn custom_fields(&self) -> &HashMap<String, String> {
        &self.custom
    }
}

impl FromStr for IcyMetadata {
    type Err = EmptyMetadataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut metadata = Self {
            stream_title: None,
            stream_url: None,
            custom: HashMap::new(),
        };

        let ParseResult {
            map,
            errors_found,
            missing_quotes_found,
        } = parse_delimited_string(s);
        if map.is_empty() {
            return Err(EmptyMetadataError(s.to_string()));
        }

        let mut fields_found = 0;
        let mut stray_values_found = false;
        for (key, value) in map {
            fields_found += 1;
            match key.to_ascii_lowercase().as_str() {
                "streamtitle" => {
                    metadata.stream_title = Some(value.to_string());
                }
                "streamurl" => {
                    metadata.stream_url = Some(value.to_string());
                }
                _ => {
                    metadata.custom.insert(key.to_string(), value.to_string());
                    stray_values_found = true;
                }
            }
        }
        // Escaping characters like quotes, semicolons, and equal signs within the metadata string
        // doesn't seem to be well-defined Here we try to handle the scenario where a stray
        // semicolon in one of the values messes with the parsing by relying on the fact
        // that StreamTitle and StreamUrl should be the only valid keys
        if errors_found || stray_values_found {
            let semicolon_count = s.chars().filter(|c| *c == ';').count();
            if semicolon_count > fields_found || missing_quotes_found {
                warn!(
                    metadata_string = s,
                    "found possibly malformed metadata, attempting to resolve any unescaped fields",
                );
                handle_unescaped_values(s, &mut metadata);
            }
        }

        Ok(metadata)
    }
}

fn handle_unescaped_values(s: &str, metadata: &mut IcyMetadata) {
    let lower_string = s.to_ascii_lowercase();
    let stream_title_index = lower_string.find("streamtitle=");
    let stream_url_index = lower_string.find("streamurl=");

    let (stream_title, stream_url) = match (stream_title_index, stream_url_index) {
        (Some(stream_title_index), Some(stream_url_index)) => {
            let (stream_title, stream_url) = if stream_title_index < stream_url_index {
                let stream_title = &s[stream_title_index..stream_url_index];
                let stream_url = &s[stream_url_index..];
                (stream_title, stream_url)
            } else {
                let stream_url = &s[stream_url_index..stream_title_index];
                let stream_title = &s[stream_title_index..];
                (stream_title, stream_url)
            };
            (Some(stream_title), Some(stream_url))
        }
        (Some(stream_title_index), None) => {
            let stream_title = &s[stream_title_index..];
            (Some(stream_title), None)
        }
        (None, Some(stream_url_index)) => {
            let stream_url = &s[stream_url_index..];
            (None, Some(stream_url))
        }
        (None, None) => (None, None),
    };

    if let Some(stream_title) = stream_title {
        metadata.stream_title = parse_value_if_valid(stream_title);
    };

    if let Some(stream_url) = stream_url {
        metadata.stream_url = parse_value_if_valid(stream_url);
    };
}

#[derive(Debug)]
struct MetadataSize {
    size: usize,
    count: usize,
}

#[derive(Debug)]
struct MetadataSizeQueue {
    inner: VecDeque<MetadataSize>,
    cache_size: usize,
}

impl MetadataSizeQueue {
    fn push(&mut self, size: usize) {
        if let Some(last) = self.inner.back_mut() {
            if last.size == size {
                last.count += 1;
                return;
            }
        }
        self.inner.push_back(MetadataSize { size, count: 1 });
        if self.inner.len() >= self.cache_size {
            self.inner.pop_front();
        }
    }

    fn pop(&mut self) -> Option<usize> {
        let last = self.inner.back_mut()?;
        last.count -= 1;
        let size = last.size;
        if last.count == 0 {
            self.inner.pop_back();
        }
        Some(size)
    }

    fn peek(&self) -> Option<usize> {
        self.inner.back().map(|b| b.size)
    }
}
