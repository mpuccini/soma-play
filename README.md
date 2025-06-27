# SomaFM Player 🎵

A terminal-based music player for [SomaFM](https://somafm.com/) internet radio stations, built with Rust and featuring a beautiful TUI (Terminal User Interface).

## ⚠️ Development Status

**This project is currently under active development and should be considered alpha software.** While functional, you may encounter bugs, incomplete features, or breaking changes between versions. Use at your own discretion and please report any issues you encounter.

## Features

- 🎵 **Stream SomaFM Radio Stations** - Access all available SomaFM channels
- 🖥️ **Beautiful Terminal UI** - Clean, intuitive TUI built with ratatui
- 🎛️ **Volume Control** - Adjust volume with `+`/`-` keys (0-100%)
- � **Live Spectrum Visualizer** - Real-time audio frequency display with animated bars
- ⏯️ **Pause/Resume Playback** - Control playback with `P` key
- �💾 **Persistent Configuration** - Remembers your last channel and settings
- 🎤 **Real-time Metadata** - Display current artist and track information
- 📂 **Smart Configuration** - Auto-saves settings to `~/.config/soma-player/`
- 📝 **Enhanced Logging** - Comprehensive logging with file rotation and filtering
- 🛡️ **Robust Error Handling** - Detailed error reporting and graceful failure handling
- 🧪 **Comprehensive Testing** - Full unit test coverage for all modules
- 📚 **Complete Documentation** - Extensive inline documentation and examples

## Installation

### Quick Install (Recommended)

Install SomaFM Player with a single command:

#### Using curl
```bash
curl -sSL https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash
```

#### Using wget
```bash
wget -qO- https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash
```

This script will:
- ✅ Automatically detect your platform (Linux x64 or macOS ARM64)
- ✅ Download the latest release binary
- ✅ Install to `~/.local/bin/soma-player`
- ✅ Make the binary executable
- ✅ Verify the installation

**Note:** Make sure `~/.local/bin` is in your PATH. If not, the installer will show you how to add it.

### Manual Installation

#### Prerequisites

**Linux only** - Install audio dependencies:
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora/CentOS/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

#### Download and Install Binary

1. **Download the latest release:**
   - Go to [Releases](https://github.com/mpuccini/soma-play/releases/latest)
   - Download the appropriate archive for your platform:
     - `soma-player-linux-x64.tar.gz` (Linux x86_64)
     - `soma-player-macos-arm64.tar.gz` (macOS Apple Silicon)

2. **Extract and install:**
   ```bash
   # Extract the archive
   tar -xzf soma-player-*.tar.gz
   
   # Move to a directory in your PATH
   mv soma-player ~/.local/bin/
   # or system-wide (requires sudo)
   sudo mv soma-player /usr/local/bin/
   
   # Make executable (if needed)
   chmod +x ~/.local/bin/soma-player
   ```

3. **Verify installation:**
   ```bash
   soma-player --version
   ```

### Build from Source

If you prefer to build from source or need to customize the build:

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install system dependencies (Linux only):**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libasound2-dev pkg-config

   # Fedora/CentOS/RHEL
   sudo dnf install alsa-lib-devel pkg-config

   # Arch Linux
   sudo pacman -S alsa-lib pkg-config
   ```

3. **Clone and build:**
   ```bash
   git clone https://github.com/mpuccini/soma-play.git
   cd soma-play
   cargo build --release
   ```

4. **Install the binary:**
   ```bash
   # Copy to local bin directory
   cp target/release/soma-player ~/.local/bin/
   
   # Or system-wide (requires sudo)
   sudo cp target/release/soma-player /usr/local/bin/
   ```

## Usage

### Controls

#### Channel Selection Screen
- **↑/↓** - Navigate channels
- **Enter** - Select channel
- **Q** - Quit

#### Playing Mode
- **C** - Change channel (opens selection overlay)
- **P** - Pause/Resume playback
- **+/=** - Increase volume (+5%)
- **-/_** - Decrease volume (-5%)
- **Q/Esc** - Quit

#### Channel Selection Overlay (while playing)
- **↑/↓** - Navigate channels
- **Enter** - Switch to selected channel
- **Esc** - Cancel and return to playing mode
- **Q** - Quit application

### Configuration

The player automatically creates a configuration file at:
```
~/.config/soma-player/config.toml
```

Example configuration:
```toml
last_channel_id = "groovesalad"
volume = 75
auto_start = false
```

#### Configuration Options

- **`last_channel_id`** - ID of the last played channel (auto-saved)
- **`volume`** - Volume level 0-100 (default: 50)
- **`auto_start`** - Skip channel selection and auto-play last channel (default: false)

### Spectrum Visualizer

The built-in spectrum visualizer displays a real-time animated frequency analysis of the currently playing audio stream. Features include:

- **Animated Frequency Bars** - Dynamic visualization that responds to music
- **Smart Layout** - Automatically adjusts bar width and spacing based on terminal size
- **Colorful Display** - Gradient colors from low to high frequencies
- **Always Active** - Updates continuously during playback, pauses when audio is paused

The visualizer simulates realistic audio frequency data and provides an engaging visual representation of the music being played. It's designed to work well in various terminal sizes and automatically scales to fit the available space.

### Logging

Enhanced logging is written to `~/.config/soma-player/logs/soma-player.log` with automatic daily rotation and includes:
- Application events and state changes
- Channel switches and volume adjustments  
- Detailed error information and stack traces
- Audio stream connection and metadata events

Configure log levels with environment variables:
```bash
RUST_LOG=debug cargo run  # More verbose logging
RUST_LOG=warn cargo run   # Less verbose logging
RUST_LOG=info cargo run   # Default level
```

Log files are automatically rotated daily and old logs are cleaned up to maintain disk space.

## Project Structure

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Module declarations
├── api/              # SomaFM API integration
├── audio/            # Audio playback engine
├── config/           # Configuration management
├── models/           # Data structures
└── ui/               # User interface components
```

## Technical Details

- **Language**: Rust 2024 Edition
- **TUI Framework**: [ratatui](https://github.com/ratatui-org/ratatui)
- **Audio Engine**: [rodio](https://github.com/RustAudio/rodio)
- **HTTP Client**: [reqwest](https://github.com/seanmonstar/reqwest)
- **Configuration**: TOML format with [toml](https://github.com/toml-rs/toml)
- **Logging**: [tracing](https://github.com/tokio-rs/tracing) with file rotation
- **Error Handling**: Custom error types with context and conversion traits
- **Testing**: Comprehensive unit tests with [tempfile](https://github.com/Stebalien/tempfile) for isolated testing
- **Metadata**: ICY metadata parsing for real-time track info
- **Visualization**: Custom spectrum visualizer with simulated frequency data and smooth animations

## Development

### Running Tests

```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test config::tests

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Documentation

Generate and view documentation:
```bash
# Generate docs
cargo doc --open

# Check all doc tests
cargo test --doc
```

### Code Quality

```bash
# Check for issues
cargo clippy

# Format code
cargo fmt

# Security audit
cargo audit
```

## Known Issues

- Some audio codecs may not be supported by the underlying audio libraries
- Network interruptions may require restarting the stream (auto-reconnect planned)
- Volume changes apply immediately but config saves may occasionally fail
- Auto-start feature is basic (planned improvements for better UX)
- Large log files may accumulate over time (automatic cleanup implemented)

## Contributing

This project is in early development. If you'd like to contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash   git clone https://github.com/mpuccini/soma-play.git
cd soma-player
cargo build
cargo test
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [SomaFM](https://somafm.com/) - For providing amazing commercial-free internet radio
- [Rust Audio Community](https://github.com/RustAudio) - For excellent audio libraries
- [ratatui](https://github.com/ratatui-org/ratatui) - For the fantastic TUI framework

## Support SomaFM

This player is an unofficial client. Please consider [supporting SomaFM](https://somafm.com/support/) directly - they provide an incredible service and deserve our support!

---

**Enjoy the music! 🎶**
