use std::collections::HashMap;
use std::num::NonZeroUsize;

use http::{HeaderMap, HeaderValue};

use crate::parse::{ParseResult, parse_delimited_string};

/// Header name to request icy metadata.
pub const ICY_METADATA_HEADER: &str = "Icy-MetaData";

/// Appends the `Icy-MetaData` header to the `header_map`.
pub fn add_icy_metadata_header(header_map: &mut HeaderMap) {
    header_map.append(
        ICY_METADATA_HEADER,
        "1".parse().expect("valid header value"),
    );
}

/// Trait for requesting icy metadata from an HTTP request builder
pub trait RequestIcyMetadata {
    /// Appends the `Icy-MetaData` header to the request's header map
    fn request_icy_metadata(self) -> Self;
}

#[cfg(feature = "reqwest")]
impl RequestIcyMetadata for reqwest::ClientBuilder {
    fn request_icy_metadata(self) -> Self {
        let mut header_map = HeaderMap::new();
        add_icy_metadata_header(&mut header_map);
        self.default_headers(header_map)
    }
}

#[cfg(feature = "reqwest")]
impl RequestIcyMetadata for reqwest::RequestBuilder {
    fn request_icy_metadata(self) -> Self {
        let mut header_map = HeaderMap::new();
        add_icy_metadata_header(&mut header_map);
        self.headers(header_map)
    }
}

/// Icy metadata found within HTTP response headers.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IcyHeaders {
    bitrate: Option<u32>,
    sample_rate: Option<u32>,
    genre: Option<String>,
    name: Option<String>,
    station_url: Option<String>,
    description: Option<String>,
    public: Option<bool>,
    notice1: Option<String>,
    notice2: Option<String>,
    loudness: Option<f32>,
    metadata_interval: Option<NonZeroUsize>,
    audio_info: Option<IcyAudioInfo>,
}

fn find_header<'a>(search: &[&'a str], headers: &'a HeaderMap) -> Option<&'a HeaderValue> {
    for header in search {
        if let Some(val) = headers.get(*header) {
            return Some(val);
        }
    }
    None
}

impl IcyHeaders {
    /// Parse any icy metadata contained in the `headers`.
    pub fn parse_from_headers(headers: &HeaderMap) -> Self {
        // Most header names taken from here https://github.com/xiph/Icecast-Server/blob/master/src/source.c
        Self {
            bitrate: find_header(&["ice-bitrate", "icy-br", "x-audiocast-bitrate"], headers)
                .and_then(|val| val.to_str().ok())
                // sometimes there are multiple values here, we'll just take the first one
                .and_then(|val| val.split(',').next()?.parse().ok()),
            // Note: this isn't included in the Icecast-Server repo, but I've seen a few servers
            // include icy-sr as a header. Unclear if the other aliases here are
            // actually used at all
            sample_rate: find_header(
                &["ice-samplerate", "icy-sr", "x-audiocast-samplerate"],
                headers,
            )
            .and_then(|val| val.to_str().ok()?.parse().ok()),
            genre: find_header(&["ice-genre", "icy-genre", "x-audiocast-genre"], headers)
                .and_then(|val| Some(val.to_str().ok()?.to_string())),
            name: find_header(&["ice-name", "icy-name", "x-audiocast-name"], headers)
                .and_then(|val| Some(val.to_str().ok()?.to_string())),
            description: find_header(
                &[
                    "ice-description",
                    "icy-description",
                    "x-audiocast-description",
                ],
                headers,
            )
            .and_then(|val| Some(val.to_str().ok()?.to_string())),
            station_url: find_header(&["ice-url", "icy-url", "x-audiocast-url"], headers)
                .and_then(|val| Some(val.to_str().ok()?.to_string())),
            notice1: find_header(
                &["ice-notice1", "icy-notice1", "x-audiocast-notice1"],
                headers,
            )
            .and_then(|val| Some(val.to_str().ok()?.to_string())),
            notice2: find_header(
                &["ice-notice2", "icy-notice2", "x-audiocast-notice2"],
                headers,
            )
            .and_then(|val| Some(val.to_str().ok()?.to_string())),
            // I can't find any documentation on this header, but some servers return it
            loudness: find_header(&["X-Loudness"], headers)
                .and_then(|val| val.to_str().ok()?.to_string().parse().ok()),
            public: find_header(
                &["ice-public", "icy-pub", "icy-public", "x-audiocast-public"],
                headers,
            )
            .and_then(|val| Some(val.to_str().ok()?.to_string()))
            .map(|public| {
                // 1 and 0 are the only supported values, but we'll look for "true" as well
                // because... why not
                public == "1" || public.to_ascii_lowercase() == "true"
            }),
            metadata_interval: headers
                .get("icy-metaint")
                .and_then(|val| NonZeroUsize::new(val.to_str().ok()?.to_string().parse().ok()?)),
            audio_info: headers.get("ice-audio-info").and_then(|val| {
                let ParseResult { map, .. } = parse_delimited_string(val.to_str().ok()?);
                Some(IcyAudioInfo::parse_from_map(map))
            }),
        }
    }

    /// Stream bitrate.
    pub fn bitrate(&self) -> Option<u32> {
        self.bitrate
    }

    /// Stream sample rate.
    pub fn sample_rate(&self) -> Option<u32> {
        self.sample_rate
    }

    /// Stream genre.
    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }

    /// Stream description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Stream name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Stream station URL.
    pub fn station_url(&self) -> Option<&str> {
        self.station_url.as_deref()
    }

    /// You probably don't care about this, but it's here just in case.
    /// If it's set, it might say something like `<BR>This stream requires <a href="http://www.winamp.com">Winamp</a><BR>`.
    pub fn notice1(&self) -> Option<&str> {
        self.notice1.as_deref()
    }

    /// You probably don't care about this, but it's here just in case.
    /// If it's set, it might contain the Icecast/Shoutcast server version.
    pub fn notice2(&self) -> Option<&str> {
        self.notice2.as_deref()
    }

    /// Loudness normalization info.
    pub fn loudness(&self) -> Option<f32> {
        self.loudness
    }

    /// Whether the stream is listed or not.
    pub fn public(&self) -> Option<bool> {
        self.public
    }

    /// This will only be set if the stream was requested with the `Icy-MetaInt` header set to `1`.
    /// Use the convenience functions in this crate to set this, or add the header yourself.
    /// This needs to be passed in to [`IcyMetadataReader::new`](crate::IcyMetadataReader::new) in
    /// order to read the metadata.
    pub fn metadata_interval(&self) -> Option<NonZeroUsize> {
        self.metadata_interval
    }

    /// Additional audio properties, if available.
    pub fn audio_info(&self) -> Option<&IcyAudioInfo> {
        self.audio_info.as_ref()
    }
}

/// Optional additional information about the stream audio
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IcyAudioInfo {
    sample_rate: Option<u32>,
    bitrate: Option<u32>,
    channels: Option<u16>,
    quality: Option<String>,
    custom: HashMap<String, String>,
}

impl IcyAudioInfo {
    fn parse_from_map(map: HashMap<&str, &str>) -> Self {
        let mut info = Self {
            sample_rate: None,
            bitrate: None,
            channels: None,
            quality: None,
            custom: HashMap::new(),
        };
        for (key, value) in map {
            let Ok(key) = urlencoding::decode(key) else {
                continue;
            };
            let Ok(value) = urlencoding::decode(value) else {
                continue;
            };
            match key.to_ascii_lowercase().as_str() {
                "icy-samplerate" | "ice-samplerate" | "samplerate" => {
                    info.sample_rate = value.parse().ok();
                }
                "icy-bitrate" | "ice-bitrate" | "bitrate" => {
                    info.bitrate = value.parse().ok();
                }
                "icy-channels" | "ice-channels" | "channels" => {
                    info.channels = value.parse().ok();
                }
                "icy-quality" | "ice-quality" | "quality" => {
                    info.quality = value.parse().ok();
                }
                _ => {
                    info.custom.insert(key.to_string(), value.to_string());
                }
            }
        }
        info
    }

    /// Stream sample rate.
    pub fn sample_rate(&self) -> Option<u32> {
        self.sample_rate
    }

    /// Stream bitrate.
    pub fn bitrate(&self) -> Option<u32> {
        self.bitrate
    }

    /// Number of channels in the stream.
    pub fn channels(&self) -> Option<u16> {
        self.channels
    }

    /// Stream quality.
    pub fn quality(&self) -> Option<&str> {
        self.quality.as_deref()
    }

    /// Additional properties, if available.
    pub fn custom(&self) -> &HashMap<String, String> {
        &self.custom
    }
}
