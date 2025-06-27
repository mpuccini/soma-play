# Architecture

Technical overview of SomaFM Player's internal structure and design.

## Project Structure

SomaFM Player follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs           # Application entry point and CLI handling
├── lib.rs            # Module declarations and exports
├── api/              # SomaFM API integration
│   ├── mod.rs        # API module exports
│   ├── client.rs     # HTTP client for SomaFM API
│   └── channels.rs   # Channel data structures and parsing
├── audio/            # Audio playback engine
│   ├── mod.rs        # Audio module exports
│   ├── player.rs     # Main audio player implementation
│   ├── stream.rs     # Audio stream handling
│   └── commands.rs   # Player command definitions
├── config/           # Configuration management
│   ├── mod.rs        # Config module exports
│   ├── settings.rs   # Configuration data structures
│   └── file.rs       # File I/O operations
├── models/           # Data structures and types
│   ├── mod.rs        # Models module exports
│   ├── channel.rs    # Channel data model
│   ├── player.rs     # Player state model
│   ├── errors.rs     # Custom error types
│   └── spectrum.rs   # Spectrum visualizer data
└── ui/               # Terminal User Interface
    ├── mod.rs        # UI module exports
    ├── app.rs        # Main application state
    ├── player.rs     # Player interface screen
    ├── channels.rs   # Channel selection screen
    └── spectrum.rs   # Spectrum visualizer widget
```

## Core Components

### 1. Main Application (`main.rs`)

**Responsibilities:**
- Command-line argument parsing
- Logging initialization
- Error handling setup
- Application bootstrap

**Key Features:**
- Early argument processing (--version, --help)
- Color error reporting with `color-eyre`
- Structured logging with `tracing`

```rust
// Simplified flow
fn main() -> Result<()> {
    // 1. Parse CLI args
    // 2. Setup logging
    // 3. Load configuration
    // 4. Initialize TUI
    // 5. Run main loop
}
```

### 2. API Layer (`api/`)

**Purpose:** Interface with SomaFM's public API and stream endpoints.

#### Components:
- **`client.rs`**: HTTP client wrapper around `reqwest`
- **`channels.rs`**: Channel data parsing and caching

**Key Features:**
- Async HTTP requests with retry logic
- Channel list caching
- ICY metadata parsing for "now playing" info
- Error handling for network issues

```rust
pub struct SomaClient {
    client: reqwest::Client,
    base_url: String,
}

impl SomaClient {
    pub async fn get_channels() -> Result<Vec<Channel>>;
    pub async fn get_stream_url(channel_id: &str) -> Result<String>;
}
```

### 3. Audio Engine (`audio/`)

**Purpose:** Handle audio streaming and playback control.

#### Components:
- **`player.rs`**: Main audio player using `rodio`
- **`stream.rs`**: Stream management and buffering
- **`commands.rs`**: Command patterns for player control

**Key Features:**
- Cross-platform audio output via `rodio`
- Volume control with persistent settings
- Pause/resume functionality
- Stream reconnection handling

```rust
pub enum PlayerCommand {
    Play(String),      // Stream URL
    Pause,
    Resume,
    SetVolume(f32),
    Stop,
}

pub struct AudioPlayer {
    sink: Option<rodio::Sink>,
    volume: f32,
    state: PlayerState,
}
```

### 4. Configuration (`config/`)

**Purpose:** Manage application settings and persistence.

#### Components:
- **`settings.rs`**: Configuration data structures
- **`file.rs`**: TOML file I/O operations

**Key Features:**
- TOML-based configuration
- Automatic directory creation
- Default value handling
- Type-safe configuration access

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub last_channel_id: Option<String>,
    pub volume: u8,
    pub auto_start: bool,
}
```

### 5. Data Models (`models/`)

**Purpose:** Define core data structures and business logic.

#### Components:
- **`channel.rs`**: SomaFM channel representation
- **`player.rs`**: Player state management
- **`errors.rs`**: Custom error types with context
- **`spectrum.rs`**: Spectrum analyzer data structures

**Key Features:**
- Serde serialization support
- Rich error types with stack traces
- Type safety throughout the application

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub listeners: u32,
    pub genre: String,
}
```

### 6. User Interface (`ui/`)

**Purpose:** Terminal-based user interface using `ratatui`.

#### Components:
- **`app.rs`**: Application state machine
- **`player.rs`**: Main playback interface
- **`channels.rs`**: Channel selection screens
- **`spectrum.rs`**: Real-time spectrum visualizer

**Key Features:**
- Event-driven architecture
- Responsive layout system
- Real-time spectrum visualization
- Keyboard input handling

```rust
pub enum AppState {
    ChannelSelection,
    Playing,
    ChannelOverlay,
    Quitting,
}

pub struct App {
    pub state: AppState,
    pub channels: Vec<Channel>,
    pub selected_channel: Option<usize>,
    pub player_state: PlayerState,
}
```

## Data Flow

### Application Startup
1. **CLI Parsing** → Parse command-line arguments
2. **Logging Setup** → Initialize structured logging
3. **Config Loading** → Load/create configuration file
4. **API Initialization** → Fetch channel list from SomaFM
5. **UI Startup** → Initialize terminal interface
6. **Event Loop** → Start main application loop

### Channel Selection Flow
1. **User Input** → Keyboard navigation
2. **UI Update** → Highlight selected channel
3. **Channel Selection** → User presses Enter
4. **Stream Resolution** → Get stream URL from API
5. **Audio Playback** → Start audio player
6. **State Transition** → Switch to playing mode

### Audio Playback Flow
1. **Stream Request** → HTTP request to stream URL
2. **Stream Decoding** → Audio format detection and decoding
3. **Audio Output** → Platform-specific audio output
4. **Metadata Parsing** → Extract "now playing" information
5. **UI Updates** → Display current track info
6. **Spectrum Analysis** → Generate frequency data for visualizer

## Threading Model

### Main Thread
- UI rendering and event handling
- Configuration management
- Application state coordination

### Audio Thread (via `rodio`)
- Audio decoding and playback
- Stream buffering
- Volume control

### Network Thread (via `tokio`)
- HTTP requests to SomaFM API
- Stream data fetching
- Metadata updates

## Error Handling Strategy

### Error Types
- **Network Errors**: API timeouts, connection failures
- **Audio Errors**: Codec issues, hardware problems
- **Configuration Errors**: File I/O, parsing failures
- **UI Errors**: Terminal capabilities, rendering issues

### Error Propagation
```rust
// Custom Result type with context
pub type Result<T> = std::result::Result<T, SomaError>;

// Rich error context
#[derive(Debug, thiserror::Error)]
pub enum SomaError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Audio error: {0}")]
    Audio(String),
    
    #[error("Configuration error: {0}")]
    Config(#[from] toml::de::Error),
}
```

## Performance Considerations

### Memory Management
- **Channel List Caching**: Avoid repeated API calls
- **Audio Buffering**: Optimal buffer sizes for smooth playback
- **UI Rendering**: Efficient terminal updates

### Network Optimization
- **Connection Reuse**: HTTP client connection pooling
- **Retry Logic**: Exponential backoff for failed requests
- **Stream Recovery**: Automatic reconnection on network issues

### CPU Usage
- **Spectrum Analysis**: Optimized FFT calculations
- **UI Updates**: Minimal redraws, only when necessary
- **Event Processing**: Efficient input handling

## Dependencies

### Core Dependencies
- **`ratatui`**: Terminal UI framework
- **`rodio`**: Cross-platform audio playback
- **`reqwest`**: HTTP client with async support
- **`tokio`**: Async runtime
- **`serde`**: Serialization framework
- **`toml`**: Configuration file format

### Development Dependencies
- **`tracing`**: Structured logging
- **`color-eyre`**: Enhanced error reporting
- **`tempfile`**: Testing utilities

## Testing Strategy

### Unit Tests
- Configuration loading/saving
- Error type conversions
- Data model validation
- API response parsing

### Integration Tests
- Audio playback scenarios
- UI state transitions
- Configuration persistence
- Network error handling

### Test Structure
```
tests/
├── integration/
│   ├── config_tests.rs
│   ├── audio_tests.rs
│   └── ui_tests.rs
└── common/
    └── test_helpers.rs
```

## Future Architecture Considerations

### Planned Enhancements
1. **Plugin System**: Extensible architecture for additional features
2. **Themes Support**: Customizable UI themes and colors
3. **Keyboard Customization**: User-defined keybindings
4. **Multi-instance Support**: Multiple concurrent streams
5. **Remote Control**: HTTP API for external control

### Scalability
- **Modular Design**: Easy to add new features
- **Clear Interfaces**: Well-defined module boundaries
- **Minimal Coupling**: Loose dependencies between components
- **Testable Code**: Architecture supports comprehensive testing

This architecture provides a solid foundation for a reliable, maintainable, and extensible audio streaming application.