//! Audio playback engine for SomaFM streams.
//!
//! This module handles the core audio functionality including:
//! - Connecting to audio streams
//! - Parsing ICY metadata for track information
//! - Real-time volume control
//! - Command-based playback control
//!
//! # Examples
//!
//! ```rust,no_run
//! use soma_player::audio::{play_channel, PlayerCommand};
//! use soma_player::models::{Channel, TrackInfo};
//! use tokio::sync::{mpsc, Mutex};
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = Channel {
//!     id: "groovesalad".to_string(),
//!     title: "Groove Salad".to_string(),
//!     description: "A nicely chilled plate of ambient beats".to_string(),
//!     playlists: vec![],
//! };
//!
//! let track_info = Arc::new(Mutex::new(TrackInfo::default()));
//! let (tx, rx) = mpsc::unbounded_channel();
//! let volume = Some(75);
//!
//! // Start playback
//! let result = play_channel(&channel, track_info, rx, volume).await;
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use rodio::{OutputStream, Sink, Decoder};
use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use log::{debug, error, info, warn};

use crate::models::{Channel, TrackInfo, parse_track_info};
use crate::api::parse_pls_playlist;

/// Commands that can be sent to control audio playback.
#[derive(Debug)]
pub enum PlayerCommand {
    /// Stop playback and quit
    Quit,
    /// Change volume (0-100)
    SetVolume(u8),
    /// Pause playback
    Pause,
    /// Resume playback
    Resume,
}

/// Plays a SomaFM channel's audio stream with real-time control.
///
/// This function handles the complete audio playback pipeline:
/// 1. Resolves playlist URLs (handles .pls files)
/// 2. Establishes HTTP connection with ICY metadata support
/// 3. Sets up audio decoding and playback
/// 4. Processes real-time metadata updates
/// 5. Responds to volume and control commands
///
/// # Arguments
///
/// * `channel` - The SomaFM channel to play
/// * `track_info` - Shared track information updated with ICY metadata
/// * `rx` - Command receiver for controlling playback
/// * `volume` - Optional initial volume (0-100), defaults to system volume
///
/// # Returns
///
/// Returns `Ok(false)` when playback stops normally, or an error if playback fails.
///
/// # Errors
///
/// This function can return errors for:
/// - Network connection failures
/// - Audio device initialization problems
/// - Stream decoding issues
/// - Invalid playlist formats
pub async fn play_channel(
    channel: &Channel, 
    track_info: Arc<Mutex<TrackInfo>>,
    mut rx: mpsc::UnboundedReceiver<PlayerCommand>,
    volume: Option<u8>
) -> Result<bool, String> {
    info!("Starting playback for channel: {}", channel.title);
    
    let initial_url = channel.playlists
        .iter()
        .find(|p| p.format == "mp3" && p.quality == "high") // Prefer high-quality MP3
        .or_else(|| channel.playlists.iter().find(|p| p.format == "mp3")) // Then any MP3
        .or_else(|| channel.playlists.first()) // Otherwise, just take the first available
        .map(|p| &p.url)
        .ok_or("No playable stream URL found for this channel.")?;

    debug!("Using playlist URL: {}", initial_url);

    // Check if the URL is a .pls playlist file and parse it if needed
    let stream_url = if initial_url.ends_with(".pls") {
        debug!("Parsing .pls playlist");
        parse_pls_playlist(initial_url).await.map_err(|e| {
            error!("Failed to parse .pls playlist: {}", e);
            e.to_string()
        })?
    } else {
        initial_url.to_string()
    };

    debug!("Final stream URL: {}", stream_url);

    // Create HTTP client with ICY metadata support
    let client = Client::builder()
        .request_icy_metadata()
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            format!("Failed to create HTTP client: {}", e)
        })?;

    // Create HTTP stream
    let stream = HttpStream::new(client, stream_url.parse().map_err(|e| {
        error!("Invalid URL: {}", e);
        format!("Invalid URL: {}", e)
    })?)
        .await
        .map_err(|e| {
            error!("Failed to connect to stream: {}", e);
            format!("Failed to connect to stream: {}", e)
        })?;

    // Parse ICY headers
    let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
    debug!("ICY headers: {:?}", icy_headers);
    
    // Use simpler approach with memory storage to avoid bounded storage overflow issues
    let bitrate = icy_headers.bitrate().unwrap_or(128);
    let prefetch_bytes = bitrate / 8 * 1024 * 5; // 5 seconds buffer

    debug!("Bitrate: {} kbps, prefetch: {} bytes", bitrate, prefetch_bytes);

    // Create stream downloader with memory storage (unbounded)
    let reader = StreamDownload::from_stream(
        stream,
        MemoryStorageProvider,
        Settings::default().prefetch_bytes(prefetch_bytes as u64),
    )
    .await
    .map_err(|e| {
        error!("Failed to create stream downloader: {}", e);
        format!("Failed to create stream downloader: {}", e)
    })?;

    // Create audio output
    let (_stream, handle) = OutputStream::try_default()
        .map_err(|e| {
            error!("Failed to open audio stream: {}", e);
            format!("Failed to open audio stream: {}", e)
        })?;
    let sink = Arc::new(Sink::try_new(&handle)
        .map_err(|e| {
            error!("Failed to create audio sink: {}", e);
            format!("Failed to create audio sink: {}", e)
        })?);

    // Set volume if provided (0-100 range converted to 0.0-1.0)
    if let Some(vol) = volume {
        let volume_float = (vol as f32) / 100.0;
        sink.set_volume(volume_float);
        debug!("Set volume to: {}% ({})", vol, volume_float);
    }

    // Clone track_info for the metadata callback
    let track_info_clone = Arc::clone(&track_info);

    // Create ICY metadata reader with callback
    let metadata_reader = IcyMetadataReader::new(
        reader,
        icy_headers.metadata_interval(),
        move |metadata| {
            if let Ok(md) = metadata {
                if let Some(stream_title) = md.stream_title() {
                    debug!("New metadata: {}", stream_title);
                    let new_track = parse_track_info(stream_title);
                    
                    // Update track info using try_lock to avoid blocking
                    // Don't use tokio::spawn in callback as it may not have runtime context
                    if let Ok(mut track) = track_info_clone.try_lock() {
                        *track = new_track;
                        debug!("Updated track info: {} - {}", track.artist, track.title);
                    } else {
                        // If try_lock fails, just log it - we'll try again on next metadata
                        debug!("Could not update track info (mutex locked), will retry on next metadata");
                    }
                }
            }
        },
    );

    // Create decoder and start playing
    let decoder = Decoder::new(metadata_reader)
        .map_err(|e| {
            error!("Failed to create audio decoder: {}", e);
            format!("Failed to create audio decoder: {}", e)
        })?;
    
    info!("Starting audio playback");
    sink.append(decoder);

    // Create audio playback task
    let mut audio_task = tokio::task::spawn_blocking({
        let sink_clone = Arc::clone(&sink);
        move || {
            sink_clone.sleep_until_end();
        }
    });

    // Wait for either a command or the audio task to complete
    let result = loop {
        tokio::select! {
            cmd = rx.recv() => {
                match cmd {
                    Some(PlayerCommand::Quit) | None => {
                        info!("Received quit command");
                        break Ok(false); // Quit
                    }
                    Some(PlayerCommand::SetVolume(vol)) => {
                        let volume_float = (vol as f32) / 100.0;
                        sink.set_volume(volume_float);
                        debug!("Volume changed to: {}% ({})", vol, volume_float);
                        // Continue the loop to handle more commands
                    }
                    Some(PlayerCommand::Pause) => {
                        sink.pause();
                        info!("Playback paused");
                        // Continue the loop to handle more commands
                    }
                    Some(PlayerCommand::Resume) => {
                        sink.play();
                        info!("Playback resumed");
                        // Continue the loop to handle more commands
                    }
                }
            },
            _ = &mut audio_task => {
                warn!("Audio stream ended unexpectedly");
                break Ok(false);
            }
        }
    };
    
    // Clean up
    audio_task.abort();
    info!("Audio playback stopped");
    
    result
}
