# Configuration Options

Complete reference for all configuration options in SomaFM Player.

## Configuration File

### Location
```
~/.config/soma-player/config.toml
```

### Format
The configuration file uses TOML format for human-readable settings.

### Example Configuration
```toml
# SomaFM Player Configuration

# Last played channel (auto-saved)
last_channel_id = "groovesalad"

# Audio volume (0-100)
volume = 75

# Skip channel selection on startup
auto_start = false

# Logging configuration
[logging]
level = "info"
file_rotation = true
max_files = 7

# User interface settings
[ui]
show_spectrum = true
spectrum_style = "bars"
color_scheme = "default"

# Audio settings
[audio]
buffer_size = 4096
sample_rate = 44100
reconnect_attempts = 3
```

## Core Options

### `last_channel_id`
**Type:** `String` (optional)  
**Default:** `None`  
**Auto-managed:** Yes

The ID of the last played channel. This is automatically saved when you switch channels and used to restore your session.

**Valid Values:**
- Any valid SomaFM channel ID (e.g., "groovesalad", "dronezone", "lush")
- `null` or empty for no default channel

**Example:**
```toml
last_channel_id = "groovesalad"
```

### `volume`
**Type:** `Integer`  
**Default:** `50`  
**Range:** `0-100`  
**Auto-managed:** Yes

Audio volume level as a percentage. Changes are automatically saved when you adjust volume during playback.

**Example:**
```toml
volume = 75
```

### `auto_start`
**Type:** `Boolean`  
**Default:** `false`

Skip the channel selection screen and automatically start playing the last channel on startup.

**Behavior:**
- `true`: Immediately start playing `last_channel_id` (if set)
- `false`: Show channel selection screen (default behavior)

**Example:**
```toml
auto_start = true
```

## Logging Options

### `[logging]` Section

Configure logging behavior and output.

#### `level`
**Type:** `String`  
**Default:** `"info"`  
**Values:** `"error"`, `"warn"`, `"info"`, `"debug"`, `"trace"`

Sets the minimum log level for file and console output.

**Log Levels:**
- `error`: Only critical errors
- `warn`: Warnings and errors
- `info`: General information (recommended)
- `debug`: Detailed debugging information
- `trace`: Very verbose tracing (development only)

**Example:**
```toml
[logging]
level = "debug"
```

#### `file_rotation`
**Type:** `Boolean`  
**Default:** `true`

Enable automatic log file rotation to prevent disk space issues.

**Behavior:**
- `true`: Rotate logs daily, keep last 7 days
- `false`: Single log file (grows indefinitely)

**Example:**
```toml
[logging]
file_rotation = true
```

#### `max_files`
**Type:** `Integer`  
**Default:** `7`  
**Range:** `1-365`

Number of rotated log files to keep when `file_rotation` is enabled.

**Example:**
```toml
[logging]
max_files = 14  # Keep 2 weeks of logs
```

## User Interface Options

### `[ui]` Section

Configure the terminal user interface appearance and behavior.

#### `show_spectrum`
**Type:** `Boolean`  
**Default:** `true`

Enable or disable the spectrum visualizer display.

**Example:**
```toml
[ui]
show_spectrum = false
```

#### `spectrum_style`
**Type:** `String`  
**Default:** `"bars"`  
**Values:** `"bars"`, `"line"`, `"minimal"`

Visual style for the spectrum analyzer.

**Styles:**
- `bars`: Vertical bars (default)
- `line`: Connected line graph
- `minimal`: Simple dots/characters

**Example:**
```toml
[ui]
spectrum_style = "line"
```

#### `color_scheme`
**Type:** `String`  
**Default:** `"default"`  
**Values:** `"default"`, `"dark"`, `"light"`, `"monochrome"`

Color scheme for the user interface.

**Schemes:**
- `default`: Standard colors
- `dark`: Dark theme optimized for dark terminals
- `light`: Light theme optimized for light terminals
- `monochrome`: No colors (accessibility)

**Example:**
```toml
[ui]
color_scheme = "dark"
```

## Audio Options

### `[audio]` Section

Configure audio playback and streaming behavior.

#### `buffer_size`
**Type:** `Integer`  
**Default:** `4096`  
**Range:** `1024-16384`

Audio buffer size in samples. Larger buffers reduce skipping but increase latency.

**Recommendations:**
- `1024-2048`: Low latency, may skip on slow systems
- `4096`: Balanced (recommended)
- `8192-16384`: High latency, very stable

**Example:**
```toml
[audio]
buffer_size = 8192
```

#### `sample_rate`
**Type:** `Integer`  
**Default:** `44100`  
**Values:** `22050`, `44100`, `48000`

Preferred audio sample rate in Hz.

**Common Rates:**
- `22050`: Lower quality, lower bandwidth
- `44100`: CD quality (recommended)
- `48000`: Professional audio quality

**Example:**
```toml
[audio]
sample_rate = 48000
```

#### `reconnect_attempts`
**Type:** `Integer`  
**Default:** `3`  
**Range:** `0-10`

Number of automatic reconnection attempts when stream is lost.

**Example:**
```toml
[audio]
reconnect_attempts = 5
```

## Network Options

### `[network]` Section

Configure network and streaming behavior.

#### `timeout`
**Type:** `Integer`  
**Default:** `30`  
**Range:** `5-300`

Network timeout in seconds for API requests and stream connections.

**Example:**
```toml
[network]
timeout = 60
```

#### `user_agent`
**Type:** `String`  
**Default:** `"SomaFM-Player/{version}"`

Custom User-Agent string for HTTP requests.

**Example:**
```toml
[network]
user_agent = "MyCustomPlayer/1.0"
```

#### `proxy`
**Type:** `String` (optional)  
**Default:** `None`

HTTP proxy URL for network requests.

**Example:**
```toml
[network]
proxy = "http://proxy.example.com:8080"
```

## Advanced Options

### `[advanced]` Section

Advanced configuration options for power users.

#### `config_reload_interval`
**Type:** `Integer`  
**Default:** `0`  
**Range:** `0-3600`

Automatic configuration reload interval in seconds. `0` disables auto-reload.

**Example:**
```toml
[advanced]
config_reload_interval = 300  # Reload every 5 minutes
```

#### `memory_limit`
**Type:** `Integer`  
**Default:** `100`  
**Range:** `50-1000`

Memory usage limit in MB for caching and buffers.

**Example:**
```toml
[advanced]
memory_limit = 200
```

## Environment Variable Overrides

### Logging
- `RUST_LOG`: Override log level (e.g., `RUST_LOG=debug`)
- `SOMA_LOG_FILE`: Custom log file path

### Configuration
- `SOMA_CONFIG_DIR`: Custom configuration directory
- `SOMA_CONFIG_FILE`: Custom configuration file path

### Audio
- `SOMA_AUDIO_BACKEND`: Force specific audio backend
- `SOMA_SAMPLE_RATE`: Override sample rate

### Examples
```bash
# Debug logging
RUST_LOG=debug soma-player

# Custom config location
SOMA_CONFIG_DIR=~/.soma soma-player

# Force sample rate
SOMA_SAMPLE_RATE=48000 soma-player
```

## Configuration Validation

### Automatic Validation
The application validates all configuration values on startup:

- **Range Checks**: Numeric values within valid ranges
- **Type Validation**: Correct data types for all options
- **Enum Validation**: Valid choices for string options
- **Path Validation**: Writeable paths for logs and cache

### Error Handling
Invalid configuration values result in:

1. **Warning Message**: Logged to console and file
2. **Default Fallback**: Invalid values replaced with defaults
3. **Graceful Degradation**: Application continues with valid settings

### Example Validation Error
```
WARNING: Invalid volume value '150' (must be 0-100), using default: 50
WARNING: Unknown color_scheme 'rainbow', using default: 'default'
```

## Configuration File Management

### Automatic Creation
If no configuration file exists, one is created with default values on first run.

### Backup and Restore
```bash
# Backup current configuration
cp ~/.config/soma-player/config.toml ~/.config/soma-player/config.toml.backup

# Restore from backup
cp ~/.config/soma-player/config.toml.backup ~/.config/soma-player/config.toml

# Reset to defaults (delete config file)
rm ~/.config/soma-player/config.toml
```

### Migration
Configuration is automatically migrated when upgrading between versions:

- **Backward Compatibility**: Old configurations continue to work
- **New Options**: Added with default values
- **Deprecated Options**: Ignored with warnings

## Security Considerations

### File Permissions
Configuration files are created with user-only permissions (`600`):

```bash
# Check permissions
ls -la ~/.config/soma-player/config.toml
# Should show: -rw------- (user read/write only)
```

### Sensitive Data
The configuration file contains no sensitive information:

- ✅ **Safe**: Volume levels, UI preferences, channel history
- ❌ **Never Stored**: Passwords, tokens, personal data

### Network Settings
When using proxy settings:

- **HTTP Proxies**: Supported for SomaFM API requests
- **Authentication**: Not currently supported in proxy URLs
- **HTTPS**: All SomaFM communication uses HTTPS when available

## Troubleshooting Configuration

### Common Issues

#### Configuration Not Loading
```bash
# Check file exists
ls ~/.config/soma-player/config.toml

# Check file permissions
ls -la ~/.config/soma-player/config.toml

# Check syntax with TOML validator
python3 -c "import toml; toml.load(open('~/.config/soma-player/config.toml'))"
```

#### Settings Not Persisting
1. **Check directory permissions**: `~/.config/soma-player/` must be writable
2. **Check disk space**: Ensure sufficient space for config file
3. **Check file locks**: No other processes accessing the file

#### Invalid Values Ignored
- Check logs for validation warnings
- Verify values are within valid ranges
- Ensure correct TOML syntax and types

### Reset Configuration
```bash
# Remove config file to reset to defaults
rm ~/.config/soma-player/config.toml

# Or rename to backup and recreate
mv ~/.config/soma-player/config.toml ~/.config/soma-player/config.toml.old
```

## Configuration Examples

### Minimal Configuration
```toml
# Basic setup - most users
volume = 70
auto_start = false
```

### Power User Configuration
```toml
# Advanced setup with all options
last_channel_id = "dronezone"
volume = 80
auto_start = true

[logging]
level = "info"
file_rotation = true
max_files = 14

[ui]
show_spectrum = true
spectrum_style = "bars"
color_scheme = "dark"

[audio]
buffer_size = 8192
sample_rate = 48000
reconnect_attempts = 5

[network]
timeout = 45
user_agent = "SomaPlayer-Custom/1.0"

[advanced]
config_reload_interval = 300
memory_limit = 150
```

### Development Configuration
```toml
# Setup for development and debugging
volume = 50
auto_start = false

[logging]
level = "debug"
file_rotation = false
max_files = 1

[ui]
show_spectrum = true
spectrum_style = "minimal"
color_scheme = "default"

[audio]
buffer_size = 2048
reconnect_attempts = 1

[advanced]
config_reload_interval = 60
memory_limit = 50
```

This comprehensive configuration reference covers all available options for customizing SomaFM Player to your preferences.
