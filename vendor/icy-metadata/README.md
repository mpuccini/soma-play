# icy-metadata

[![crates.io](https://img.shields.io/crates/v/icy-metadata.svg?logo=rust)](https://crates.io/crates/icy-metadata)
[![docs.rs](https://img.shields.io/docsrs/icy-metadata?logo=rust)](https://docs.rs/icy-metadata)
[![Dependency Status](https://deps.rs/repo/github/aschey/icy-metadata/status.svg?style=flat-square)](https://deps.rs/repo/github/aschey/icy-metadata)
![license](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg)
[![CI](https://github.com/aschey/icy-metadata/actions/workflows/ci.yml/badge.svg)](https://github.com/aschey/icy-metadata/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/aschey/icy-metadata/graph/badge.svg?token=cYArKUgtgH)](https://codecov.io/gh/aschey/icy-metadata)
![GitHub repo size](https://img.shields.io/github/repo-size/aschey/icy-metadata)
![Lines of Code](https://aschey.tech/tokei/github/aschey/icy-metadata)

[icy-metadata](https://github.com/aschey/icy-metadata) is a library for reading
metadata returned from Icecast-compatible web servers.

## Installation

```sh
cargo add icy-metadata
```

## Features

- `reqwest` - adds convenience methods to set icy metadata headers on
  `reqwest`'s client builder and request builder.
- `serde` - enables serialization/deserialization for metadata structs.

## Headers

Parse common Icecast headers from an HTTP response. `icy-metadata` will look for
several common aliases to find the header values.

```rust,no_run
use icy_metadata::IcyHeaders;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = reqwest::get("https://some-cool-url.com/some-file.mp3").await?;

    let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
    println!("{icy_headers:?}");

    Ok(())
}
```

## Reading information contained within the stream

Some streams have information about the current track contained within the
stream itself. Wrapping the stream in an `IcyMetadataReader` provides an
interface to read those values.

```rust,no_run
use std::error::Error;
use std::num::NonZeroUsize;

use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use stream_download::http::HttpStream;
use stream_download::http::reqwest::{self, Client};
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // We need to add a header to tell the Icecast server that we can parse the metadata
    // embedded within the stream itself.
    let client = Client::builder().request_icy_metadata().build()?;
    let stream =
        HttpStream::new(client, "https://some-cool-url.com/some-file.mp3".parse()?).await?;

    let icy_headers = IcyHeaders::parse_from_headers(stream.headers());

    // buffer 5 seconds of audio
    // bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
    let prefetch_bytes = icy_headers.bitrate().unwrap() / 8 * 1024 * 5;

    let reader = StreamDownload::from_stream(
        stream,
        // use bounded storage to keep the underlying size from growing indefinitely
        BoundedStorageProvider::new(
            MemoryStorageProvider,
            // be liberal with the buffer size, you need to make sure it holds
            // enough space to prevent any out-of-bounds reads
            NonZeroUsize::new(512 * 1024).unwrap(),
        ),
        Settings::default().prefetch_bytes(prefetch_bytes as u64),
    )
    .await?;

    let metadata_reader = IcyMetadataReader::new(
        reader,
        // Since we requested icy metadata, the metadata interval header should be
        // present in the response. This will allow us to parse the metadata
        // within the stream.
        icy_headers.metadata_interval(),
        // Print the stream metadata whenever we receive new values
        |metadata| println!("{metadata:?}\n"),
    );

    Ok(())
}
```

### Seeking within the stream

Seeking is supported with a few limitations. See the docs for
[`IcyMetadataReader`](https://docs.rs/icy_metadata/latest/icy_metadata/struct.IcyMetadataReader.html)
for details.

## Supported Rust Versions

The MSRV is currently `1.71.0`.
