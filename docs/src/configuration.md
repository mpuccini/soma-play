# Configuration

SomaFM Player uses a TOML configuration file to store settings and preferences.

## Configuration File Location

The configuration file is automatically created at:
```
~/.config/soma-player/config.toml
```

On different systems:
- **Linux**: `~/.config/soma-player/config.toml`
- **macOS**: `~/.config/soma-player/config.toml`
- **Windows**: `%APPDATA%\soma-player\config.toml`

## Default Configuration

When you first run the application, it creates a default configuration:

```toml
last_channel_id = ""
volume = 50
auto_start = false
```

## Configuration Options

### `last_channel_id`
- **Type**: String
- **Default**: `""` (empty)
- **Description**: ID of the last played channel
- **Auto-saved**: Yes, updated automatically when you change channels

**Example**:
```toml
last_channel_id = "groovesalad"
```

### `volume`
- **Type**: Integer
- **Range**: 0-100
- **Default**: 50
- **Description**: Audio volume level as a percentage
- **Auto-saved**: Yes, updated when you adjust volume with +/- keys

**Example**:
```toml
volume = 75
```

### `auto_start`
- **Type**: Boolean
- **Default**: `false`
- **Description**: Skip channel selection and automatically play the last channel
- **Behavior**: 
  - `true`: Starts playing `last_channel_id` immediately
  - `false`: Shows channel selection screen

**Example**:
```toml
auto_start = true
```

## Complete Example Configuration

```toml
# Last played channel (auto-saved)
last_channel_id = "groovesalad"

# Volume level (0-100, auto-saved)
volume = 75

# Auto-start last channel on startup
auto_start = false
```

## Logging Configuration

Logging behavior can be controlled via environment variables:

### Log Levels
Set the `RUST_LOG` environment variable:

```bash
# Debug level (very verbose)
RUST_LOG=debug soma-player

# Info level (default)
RUST_LOG=info soma-player

# Warning level (minimal logging)
RUST_LOG=warn soma-player
```

### Log Files
Log files are automatically created at:
```
~/.config/soma-player/logs/soma-player.log
```

Features:
- **Daily rotation**: New log file each day
- **Automatic cleanup**: Old logs are removed
- **Size management**: Prevents excessive disk usage

## Manual Configuration

You can manually edit the configuration file while the application is not running:

1. **Stop the application** (if running)
2. **Edit the file**:
   ```bash
   nano ~/.config/soma-player/config.toml
   ```
3. **Save and restart** the application

## Troubleshooting Configuration

### Configuration Not Saving
- Check file permissions for `~/.config/soma-player/`
- Ensure the directory is writable
- Look for error messages in the logs

### Configuration File Corrupted
If the configuration file becomes corrupted:

1. **Stop the application**
2. **Remove the config file**:
   ```bash
   rm ~/.config/soma-player/config.toml
   ```
3. **Restart the application** - it will create a new default configuration

### Reset to Defaults
To reset all settings to defaults:

```bash
# Remove the entire configuration directory
rm -rf ~/.config/soma-player/

# Restart the application
soma-player
```

## Advanced Configuration

### Multiple Profiles
While not built-in, you can use different config directories:

```bash
# Use a custom config directory
XDG_CONFIG_HOME=/path/to/custom/config soma-player
```

### System-wide Configuration
For system-wide defaults, you could:
1. Create a template configuration
2. Copy it to user directories as needed
3. Use deployment scripts for multiple users

## Configuration Schema

The configuration follows this schema:

```toml
# String: Channel ID from SomaFM API
last_channel_id = "string"

# Integer: Volume level 0-100
volume = 50

# Boolean: Auto-start behavior
auto_start = false
```

## Future Configuration Options

Planned configuration options for future releases:
- `theme` - Color theme selection
- `keybindings` - Custom keyboard shortcuts
- `network_timeout` - Connection timeout settings
- `retry_attempts` - Network retry configuration
