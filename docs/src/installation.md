# Installation

## Quick Install (Recommended)

Install SomaFM Player with a single command:

### Using curl
```bash
curl -sSL https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash
```

### Using wget
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

## Manual Installation

### Prerequisites

**Linux only** - Install audio dependencies:
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora/CentOS/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

### Download and Install Binary

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

## Build from Source

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

## Updating

To update SomaFM Player, simply run the installation command again:

```bash
curl -sSL https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash
```

This will automatically download and install the latest version.
