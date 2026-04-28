use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use stream_download::http::HttpStream;
use stream_download::http::reqwest::Client;
use stream_download::source::DecodeError;
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;

    let restart = Arc::new(AtomicBool::new(true));

    loop {
        if !restart.swap(false, Ordering::Relaxed) {
            return Ok(());
        }

        let sink = rodio::Sink::try_new(&handle)?;

        // We need to add a header to tell the Icecast server that we can parse the metadata
        // embedded within the stream itself.
        let client = Client::builder().request_icy_metadata().build()?;

        let stream =
            HttpStream::new(client, "https://ice6.somafm.com/insound-128-mp3".parse()?).await?;

        let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
        println!("Icecast headers: {icy_headers:#?}\n");
        println!("content type={:?}\n", stream.content_type());

        // buffer 5 seconds of audio
        // bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
        let prefetch_bytes = icy_headers.bitrate().unwrap() / 8 * 1024 * 5;

        let reader = match StreamDownload::from_stream(
            stream,
            // use bounded storage to keep the underlying size from growing indefinitely
            BoundedStorageProvider::new(
                MemoryStorageProvider,
                // be liberal with the buffer size, you need to make sure it holds enough space to
                // prevent any out-of-bounds reads
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default()
                .prefetch_bytes(prefetch_bytes as u64)
                .on_reconnect({
                    let restart = restart.clone();
                    move |_stream, cancellation_token| {
                        // If the stream reconnects after a network failure, the internal state of
                        // the metadata parser is likely invalid.
                        // We should cancel the current download task and re-instantiate it in order
                        // to reset everything.
                        cancellation_token.cancel();
                        restart.store(true, Ordering::Relaxed);
                    }
                }),
        )
        .await
        {
            Ok(reader) => reader,
            Err(e) => return Err(e.decode_error().await)?,
        };

        sink.append(rodio::Decoder::new(IcyMetadataReader::new(
            reader,
            // Since we requested icy metadata, the metadata interval header should be present in
            // the response. This will allow us to parse the metadata within the stream
            icy_headers.metadata_interval(),
            // Print the stream metadata whenever we receive new values
            |metadata| {
                println!("{metadata:#?}\n");
            },
        ))?);

        let handle = tokio::task::spawn_blocking(move || {
            sink.sleep_until_end();
        });
        handle.await?;
    }
}
