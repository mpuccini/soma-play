use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tokio::sync::{mpsc, Mutex};
use rodio::{OutputStream, Sink, Decoder};
use std::sync::Arc;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
    execute,
    terminal::{Clear, ClearType},
    cursor::{MoveTo},
};
use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use colored::*;
use unicode_width::UnicodeWidthStr;

const SOMAFM_API_URL: &str = "https://api.somafm.com/channels.json";

#[derive(Debug, Clone)]
struct TrackInfo {
    artist: String,
    title: String,
    updated_at: std::time::Instant,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            artist: "Unknown".to_string(),
            title: "Loading...".to_string(),
            updated_at: std::time::Instant::now(),
        }
    }
}

#[derive(Debug)]
enum PlayerCommand {
    ChangeChannel,
    Quit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Channel {
    id: String,
    title: String,
    description: String,
    playlists: Vec<Playlist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Playlist {
    url: String,
    format: String,
    quality: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SomaFmResponse {
    channels: Vec<Channel>,
}

/// Fetches the list of SomaFM channels from the API.
async fn fetch_channels() -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    println!("{} {}", "🌐".bright_blue(), "Fetching SomaFM channels...".bright_white());
    let response = reqwest::get(SOMAFM_API_URL).await?.json::<SomaFmResponse>().await?;
    println!("{} {}", "✅".bright_green(), format!("Found {} channels!", response.channels.len()).bright_white());
    Ok(response.channels)
}

/// Displays channels and prompts the user to select one.
fn select_channel(channels: &[Channel]) -> Result<&Channel, Box<dyn std::error::Error>> {
    println!("\n{}", "🎵 SomaFM Channels 🎵".bright_cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    
    for (i, channel) in channels.iter().enumerate() {
        let number = format!("{}.", i + 1).bright_yellow().bold();
        let title = channel.title.bright_green().bold();
        let desc = channel.description.white().dimmed();
        println!("{:>3} {} - {}", number, title, desc);
    }
    
    println!("{}", "═".repeat(50).cyan());
    
    loop {
        print!("{} ", "Enter channel number:".bright_blue().bold());
        io::stdout().flush()?; // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= channels.len() {
                return Ok(&channels[choice - 1]);
            }
        }
        println!("{} Please enter a number between 1 and {}.", 
                "❌ Invalid input!".bright_red().bold(), 
                channels.len());
    }
}

/// Parses a .pls playlist file and returns the first stream URL
async fn parse_pls_playlist(pls_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("{} {}", "🔍".bright_yellow(), "Parsing playlist file...".white());
    
    let client = reqwest::Client::new();
    let response = client.get(pls_url).send().await?;
    let pls_content = response.text().await?;
    
    // Parse the .pls file to find File1, File2, etc.
    for line in pls_content.lines() {
        let line = line.trim();
        if line.starts_with("File") && line.contains("=") {
            if let Some(url) = line.split('=').nth(1) {
                let url = url.trim();
                if url.starts_with("http") {
                    println!("{} {}", "🔗".bright_green(), format!("Stream URL found: {}", url).white());
                    return Ok(url.to_string());
                }
            }
        }
    }
    
    Err("No valid stream URL found in .pls playlist".into())
}

/// Parses track info from ICY stream title
fn parse_track_info(stream_title: &str) -> TrackInfo {
    // Try to split on " - " to separate artist and title
    if let Some(dash_pos) = stream_title.find(" - ") {
        let artist = stream_title[..dash_pos].trim().to_string();
        let title = stream_title[dash_pos + 3..].trim().to_string();
        
        if !artist.is_empty() && !title.is_empty() {
            return TrackInfo {
                artist,
                title,
                updated_at: std::time::Instant::now(),
            };
        }
    }
    
    // If no " - " found, use the entire string as title
    TrackInfo {
        artist: "Unknown".to_string(),
        title: stream_title.to_string(),
        updated_at: std::time::Instant::now(),
    }
}

/// Truncates text to fit within specified width, adding ellipsis if needed
fn truncate_text(text: &str, max_width: usize) -> String {
    let text_width = text.width();
    if text_width <= max_width {
        text.to_string()
    } else {
        let mut result = String::new();
        let mut current_width = 0;
        
        for ch in text.chars() {
            let ch_width = ch.width().unwrap_or(0);
            if current_width + ch_width + 3 > max_width { // +3 for "..."
                result.push_str("...");
                break;
            }
            result.push(ch);
            current_width += ch_width;
        }
        result
    }
}

/// Creates a centered text within a given width
fn center_text(text: &str, width: usize) -> String {
    let text_width = text.width();
    if text_width >= width {
        text.to_string()
    } else {
        let padding = (width - text_width) / 2;
        format!("{}{}", " ".repeat(padding), text)
    }
}

/// Displays the current playing information with colors and better formatting
fn display_status(channel: &Channel, track_info: &TrackInfo) {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();
    
    let terminal_width = 80; // Assume 80 character width
    let content_width = terminal_width - 4; // Leave 2 chars padding on each side
    
    // Header
    println!("{}", "╭".repeat(terminal_width).cyan().bold());
    println!("{}", center_text(&"🎵 SomaFM Player 🎵".bright_cyan().bold().to_string(), terminal_width));
    println!("{}", "├".repeat(terminal_width).cyan());
    
    // Channel info with colors
    let channel_line = format!("📻 {}", channel.title.bright_yellow().bold());
    println!("  {}  ", truncate_text(&channel_line, content_width));
    println!("{}", "├".repeat(terminal_width).cyan());
    
    // Track info with improved formatting
    let artist_text = if track_info.artist != "Unknown" && track_info.artist != "Loading..." {
        format!("🎤 {}", track_info.artist.bright_green().bold())
    } else {
        format!("🎤 {}", track_info.artist.dim())
    };
    
    let title_text = if track_info.title != "Loading..." {
        format!("🎵 {}", track_info.title.bright_white().bold())
    } else {
        format!("🎵 {}", track_info.title.dim())
    };
    
    println!("  {}  ", truncate_text(&artist_text, content_width));
    println!("  {}  ", truncate_text(&title_text, content_width));
    
    // Status indicator
    let status = if track_info.title != "Loading..." {
        "🔊 Playing".bright_green()
    } else {
        "⏳ Connecting...".yellow()
    };
    println!("  {}  ", status);
    
    println!("{}", "├".repeat(terminal_width).cyan());
    
    // Controls section with colors
    println!("  {}  ", "Controls:".bright_blue().bold());
    println!("  {}  {}", "C".bright_cyan().bold(), "- Change channel".white());
    println!("  {}  {}", "Q".bright_red().bold(), "- Quit player".white());
    
    println!("{}", "╰".repeat(terminal_width).cyan().bold());
    println!(); // Extra line for spacing
    
    io::stdout().flush().unwrap();
}

/// Plays the selected SomaFM channel's stream with real-time control
async fn play_channel(
    channel: &Channel, 
    track_info: Arc<Mutex<TrackInfo>>,
    mut rx: mpsc::UnboundedReceiver<PlayerCommand>
) -> Result<bool, Box<dyn std::error::Error>> {
    let initial_url = channel.playlists
        .iter()
        .find(|p| p.format == "mp3" && p.quality == "high") // Prefer high-quality MP3
        .or_else(|| channel.playlists.iter().find(|p| p.format == "mp3")) // Then any MP3
        .or_else(|| channel.playlists.first()) // Otherwise, just take the first available
        .map(|p| &p.url)
        .ok_or("No playable stream URL found for this channel.")?;

    // Check if the URL is a .pls playlist file and parse it if needed
    let stream_url = if initial_url.ends_with(".pls") {
        parse_pls_playlist(initial_url).await?
    } else {
        initial_url.to_string()
    };

    // Create HTTP client with ICY metadata support
    let client = Client::builder()
        .request_icy_metadata()
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Create HTTP stream
    let stream = HttpStream::new(client, stream_url.parse()?)
        .await
        .map_err(|e| format!("Failed to connect to stream: {}", e))?;

    // Parse ICY headers
    let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
    
    // Use simpler approach with memory storage to avoid bounded storage overflow issues
    let bitrate = icy_headers.bitrate().unwrap_or(128);
    let prefetch_bytes = bitrate / 8 * 1024 * 5; // 5 seconds buffer

    // Create stream downloader with memory storage (unbounded)
    let reader = StreamDownload::from_stream(
        stream,
        MemoryStorageProvider,
        Settings::default().prefetch_bytes(prefetch_bytes as u64),
    )
    .await
    .map_err(|e| format!("Failed to create stream downloader: {}", e))?;

    // Create audio output
    let (_stream, handle) = OutputStream::try_default()
        .map_err(|e| format!("Failed to open audio stream: {}", e))?;
    let sink = Sink::try_new(&handle)
        .map_err(|e| format!("Failed to create audio sink: {}", e))?;

    // Clone track_info for the metadata callback
    let track_info_clone = Arc::clone(&track_info);

    // Create ICY metadata reader with callback
    let metadata_reader = IcyMetadataReader::new(
        reader,
        icy_headers.metadata_interval(),
        move |metadata| {
            if let Ok(md) = metadata {
                if let Some(stream_title) = md.stream_title() {
                    let new_track = parse_track_info(stream_title);
                    
                    // Update track info in a non-blocking way
                    if let Ok(mut track) = track_info_clone.try_lock() {
                        *track = new_track;
                    }
                }
            }
        },
    );

    // Create decoder and start playing
    let decoder = Decoder::new(metadata_reader)
        .map_err(|e| format!("Failed to create audio decoder: {}", e))?;
    
    sink.append(decoder);

    // Create audio playback task
    let mut audio_task = tokio::task::spawn_blocking(move || {
        sink.sleep_until_end();
    });

    // Wait for either a command or the audio task to complete
    let result = tokio::select! {
        cmd = rx.recv() => {
            match cmd {
                Some(PlayerCommand::ChangeChannel) => {
                    Ok(true) // Change channel
                },
                Some(PlayerCommand::Quit) | None => {
                    Ok(false) // Quit
                }
            }
        },
        _ = &mut audio_task => {
            // Audio ended
            Ok(false)
        }
    };
    
    // Clean up
    audio_task.abort();
    
    result
}

/// Handles keyboard input in a separate task
async fn handle_keyboard_input(tx: mpsc::UnboundedSender<PlayerCommand>) {
    loop {
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        let _ = tx.send(PlayerCommand::ChangeChannel);
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        let _ = tx.send(PlayerCommand::Quit);
                        break;
                    }
                    _ => {}
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Don't enable raw mode yet - we need normal input for channel selection
    let result = run_player().await;
    result
}

async fn run_player() -> Result<(), Box<dyn std::error::Error>> {
    let channels = fetch_channels().await?;
    let track_info = Arc::new(Mutex::new(TrackInfo::default()));
    
    loop {
        // Use normal terminal mode for channel selection
        let selected_channel = select_channel(&channels)?;
        
        // Now enable raw mode for the player interface
        enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
        
        // Wrap the playing logic to ensure raw mode is always disabled
        let should_change = match play_session(selected_channel, Arc::clone(&track_info)).await {
            Ok(result) => {
                disable_raw_mode().ok(); // Ignore error when disabling
                result
            }
            Err(e) => {
                disable_raw_mode().ok(); // Ignore error when disabling
                return Err(e);
            }
        };
        
        if !should_change {
            // User chose to quit
            break;
        }
        
        // Reset track info for next channel
        {
            let mut track = track_info.lock().await;
            *track = TrackInfo::default();
        }
    }
    
    // Clear screen before exit
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    println!("Thanks for using SomaFM Player! 🎵");
    
    Ok(())
}

async fn play_session(
    selected_channel: &Channel,
    track_info: Arc<Mutex<TrackInfo>>
) -> Result<bool, Box<dyn std::error::Error>> {
    // Clear screen and show initial status
    {
        let track = track_info.lock().await;
        display_status(selected_channel, &track);
    }
    
    // Create communication channel for this play session
    let (tx, rx) = mpsc::unbounded_channel();
    
    // Start keyboard input handler
    let keyboard_task = tokio::spawn(handle_keyboard_input(tx));
    
    // Create a task to update the display periodically
    let track_info_clone = Arc::clone(&track_info);
    let channel_clone = selected_channel.clone();
    let display_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
        loop {
            interval.tick().await;
            let track = track_info_clone.lock().await;
            display_status(&channel_clone, &track);
        }
    });
    
    // Play the channel
    let should_change = play_channel(selected_channel, Arc::clone(&track_info), rx).await?;
    
    // Clean up tasks
    keyboard_task.abort();
    display_task.abort();
    
    Ok(should_change)
}
