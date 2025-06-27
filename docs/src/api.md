# API Reference

Developer reference for SomaFM Player's internal APIs and data structures.

> **Note**: This page covers the internal Rust API. For the complete generated API documentation with full type information, see the [API Documentation](../api/) section.

## Core Modules

### `soma_player::config`

Configuration management for persistent settings.

#### `Config`
```rust
pub struct Config {
    pub last_channel_id: Option<String>,
    pub volume: u8,
    pub auto_start: bool,
}
```

**Methods:**
- `Config::load() -> Result<Config>` - Load configuration from file
- `Config::save(&self) -> Result<()>` - Save configuration to file
- `Config::default_path() -> PathBuf` - Get default config file path

**Example:**
```rust
use soma_player::config::Config;

let mut config = Config::load()?;
config.volume = 75;
config.save()?;
```

### `soma_player::models`

Core data structures and types.

#### `Channel`
```rust
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub listeners: u32,
    pub genre: String,
}
```

Represents a SomaFM radio channel with metadata.

#### `PlayerState`
```rust
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
    Buffering,
    Error(String),
}
```

Current state of the audio player.

#### `SomaError`
```rust
pub enum SomaError {
    Network(reqwest::Error),
    Audio(String),
    Config(toml::de::Error),
    Io(std::io::Error),
    Parse(String),
}
```

Comprehensive error types with context information.

### `soma_player::api`

SomaFM API integration and HTTP client.

#### `SomaClient`
```rust
pub struct SomaClient {
    // Internal HTTP client
}
```

**Methods:**
- `SomaClient::new() -> Self` - Create new API client
- `get_channels(&self) -> Result<Vec<Channel>>` - Fetch available channels
- `get_stream_url(&self, channel_id: &str) -> Result<String>` - Get stream URL

**Example:**
```rust
use soma_player::api::SomaClient;

let client = SomaClient::new();
let channels = client.get_channels().await?;
let stream_url = client.get_stream_url("groovesalad").await?;
```

### `soma_player::audio`

Audio playback engine and controls.

#### `AudioPlayer`
```rust
pub struct AudioPlayer {
    // Audio sink and state
}
```

**Methods:**
- `AudioPlayer::new() -> Result<Self>` - Create new audio player
- `play(&mut self, url: String) -> Result<()>` - Start playing stream
- `pause(&mut self) -> Result<()>` - Pause playback
- `resume(&mut self) -> Result<()>` - Resume playback
- `set_volume(&mut self, volume: f32) -> Result<()>` - Set volume (0.0-1.0)
- `stop(&mut self) -> Result<()>` - Stop playback

#### `PlayerCommand`
```rust
pub enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
    SetVolume(f32),
    Stop,
}
```

Commands for controlling audio playback.

**Example:**
```rust
use soma_player::audio::{AudioPlayer, PlayerCommand};

let mut player = AudioPlayer::new()?;
player.play("http://stream.url".to_string())?;
player.set_volume(0.75)?;
```

### `soma_player::ui`

Terminal user interface components.

#### `App`
```rust
pub struct App {
    pub state: AppState,
    pub channels: Vec<Channel>,
    pub selected_channel: Option<usize>,
    // ... other fields
}
```

Main application state container.

#### `AppState`
```rust
pub enum AppState {
    ChannelSelection,
    Playing,
    ChannelOverlay,
    Quitting,
}
```

Current UI state for the application.

**Methods:**
- `App::new(channels: Vec<Channel>) -> Self` - Create new app instance
- `handle_input(&mut self, event: KeyEvent) -> Result<()>` - Process keyboard input
- `update(&mut self) -> Result<()>` - Update application state
- `should_quit(&self) -> bool` - Check if app should exit

## Data Flow APIs

### Configuration Flow
```rust
// Load configuration
let config = Config::load().unwrap_or_default();

// Use configuration
if config.auto_start {
    // Auto-start last channel
}

// Save changes
config.volume = new_volume;
config.save()?;
```

### Channel Management
```rust
// Fetch channels from API
let client = SomaClient::new();
let channels = client.get_channels().await?;

// Select channel
let selected = &channels[index];
let stream_url = client.get_stream_url(&selected.id).await?;
```

### Audio Control
```rust
// Initialize player
let mut player = AudioPlayer::new()?;

// Control playback
player.play(stream_url)?;
player.set_volume(0.8)?;
player.pause()?;
player.resume()?;
```

### UI Event Handling
```rust
// Create app
let mut app = App::new(channels);

// Main event loop
loop {
    if let Ok(event) = event::read() {
        app.handle_input(event)?;
        app.update()?;
        
        if app.should_quit() {
            break;
        }
    }
}
```

## Error Handling

### Result Types
All fallible operations return `Result<T, SomaError>`:

```rust
pub type Result<T> = std::result::Result<T, SomaError>;
```

### Error Context
Errors include rich context information:

```rust
use soma_player::Result;

fn example_function() -> Result<()> {
    config.load()
        .map_err(|e| SomaError::Config(e))?;
    Ok(())
}
```

### Error Conversion
Automatic conversion from standard error types:

```rust
impl From<std::io::Error> for SomaError {
    fn from(err: std::io::Error) -> Self {
        SomaError::Io(err)
    }
}
```

## Async Patterns

### API Calls
```rust
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SomaClient::new();
    let channels = client.get_channels().await?;
    Ok(())
}
```

### Non-blocking Operations
```rust
// Spawn background task for API calls
let handle = tokio::spawn(async move {
    client.get_channels().await
});

// Continue with UI work
// ...

// Get result when needed
let channels = handle.await??;
```

## Testing APIs

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_save_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        
        let config = Config {
            volume: 75,
            last_channel_id: Some("groovesalad".to_string()),
            auto_start: true,
        };
        
        config.save_to_path(&config_path).unwrap();
        let loaded = Config::load_from_path(&config_path).unwrap();
        
        assert_eq!(config.volume, loaded.volume);
    }
}
```

### Mock Objects
```rust
// Mock API client for testing
pub struct MockSomaClient {
    channels: Vec<Channel>,
}

impl MockSomaClient {
    pub fn new(channels: Vec<Channel>) -> Self {
        Self { channels }
    }
}

#[async_trait]
impl ApiClient for MockSomaClient {
    async fn get_channels(&self) -> Result<Vec<Channel>> {
        Ok(self.channels.clone())
    }
}
```

## Extension Points

### Custom Audio Backends
```rust
pub trait AudioBackend {
    fn play(&mut self, url: String) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn set_volume(&mut self, volume: f32) -> Result<()>;
}

// Implement for different audio systems
impl AudioBackend for RodioBackend { /* ... */ }
impl AudioBackend for PulseAudioBackend { /* ... */ }
```

### Custom UI Themes
```rust
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub background_color: Color,
    pub accent_color: Color,
}

impl App {
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}
```

## Performance APIs

### Caching
```rust
use std::collections::HashMap;

pub struct ChannelCache {
    cache: HashMap<String, Vec<Channel>>,
    ttl: Duration,
}

impl ChannelCache {
    pub fn get_or_fetch(&mut self, key: &str) -> Result<Vec<Channel>> {
        // Return cached data or fetch new
    }
}
```

### Metrics
```rust
pub struct Metrics {
    pub api_calls: u64,
    pub cache_hits: u64,
    pub audio_buffer_underruns: u64,
}

impl App {
    pub fn get_metrics(&self) -> Metrics {
        // Return current metrics
    }
}
```

## Constants and Configuration

### Default Values
```rust
pub const DEFAULT_VOLUME: u8 = 50;
pub const DEFAULT_CONFIG_DIR: &str = ".config/soma-player";
pub const API_BASE_URL: &str = "https://somafm.com";
pub const STREAM_TIMEOUT: Duration = Duration::from_secs(30);
```

### Build Information
```rust
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TARGET: &str = env!("TARGET");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
```

## Feature Flags

### Conditional Compilation
```rust
#[cfg(feature = "spectrum-analyzer")]
pub mod spectrum {
    // Spectrum analyzer functionality
}

#[cfg(not(feature = "spectrum-analyzer"))]
pub mod spectrum {
    // Stub implementation
}
```

This API reference provides the essential interfaces for working with SomaFM Player's codebase. For complete type information and implementation details, refer to the generated [API Documentation](../api/).
