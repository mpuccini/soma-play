# Command Line

Command-line interface and options for SomaFM Player.

## Basic Usage

### Start the Application
```bash
soma-player
```

This starts the application with the default terminal user interface.

### Quick Help
```bash
soma-player --help
```

### Version Information
```bash
soma-player --version
```

## Command-Line Options

### Global Options

#### `--help`, `-h`
Display help information and exit.

```bash
soma-player --help
soma-player -h
```

**Output:**
```
SomaFM Player 0.1.6
A terminal-based music player for SomaFM internet radio

USAGE:
    soma-player [OPTIONS] [COMMAND]

OPTIONS:
    -h, --help       Print help information
    -v, --version    Print version information
    -c, --config     Custom configuration file path
    -l, --log-level  Override log level [debug|info|warn|error]
    -q, --quiet      Suppress non-essential output
    -V, --verbose    Enable verbose output

COMMANDS:
    play     Start playing a specific channel
    list     List available channels
    config   Manage configuration
    help     Print this message or the help of a given subcommand
```

#### `--version`, `-v`
Display version information and build details.

```bash
soma-player --version
```

**Output:**
```
soma-player 0.1.6
Build: release
Target: x86_64-unknown-linux-gnu
Rustc: 1.75.0
```

#### `--config`, `-c`
Specify a custom configuration file path.

```bash
soma-player --config /path/to/custom/config.toml
soma-player -c ~/.config/soma-alt/config.toml
```

**Default:** `~/.config/soma-player/config.toml`

#### `--log-level`, `-l`
Override the log level for this session.

```bash
soma-player --log-level debug
soma-player -l warn
```

**Valid Levels:** `error`, `warn`, `info`, `debug`, `trace`

#### `--quiet`, `-q`
Suppress non-essential output (warnings, info messages).

```bash
soma-player --quiet
```

**Effect:** Only errors are displayed to the console.

#### `--verbose`, `-V`
Enable verbose output for debugging.

```bash
soma-player --verbose
```

**Effect:** Equivalent to `--log-level debug` with additional startup information.

## Subcommands

### `play` - Direct Channel Playback

Start playing a specific channel immediately without the UI.

```bash
soma-player play <CHANNEL_ID>
```

**Examples:**
```bash
# Play Groove Salad
soma-player play groovesalad

# Play Drone Zone
soma-player play dronezone

# Play with custom volume
soma-player play groovesalad --volume 75
```

#### Options for `play`

##### `--volume`, `-V`
Set initial volume level (0-100).

```bash
soma-player play groovesalad --volume 80
soma-player play dronezone -V 60
```

##### `--no-ui`
Play without launching the terminal interface (headless mode).

```bash
soma-player play groovesalad --no-ui
```

**Use Cases:**
- Background music in scripts
- System service integration
- Remote/headless servers

##### `--duration`, `-d`
Play for a specific duration then exit.

```bash
soma-player play groovesalad --duration 30m
soma-player play dronezone -d 1h
```

**Time Formats:**
- `30s` - 30 seconds
- `5m` - 5 minutes  
- `2h` - 2 hours
- `90` - 90 seconds (default unit)

### `list` - Channel Information

Display available SomaFM channels.

```bash
soma-player list
```

**Output:**
```
Available SomaFM Channels:

groovesalad     Groove Salad              1,234 listeners
dronezone       Drone Zone                  987 listeners
lush            Lush                        756 listeners
folkfwd         Folk Forward                543 listeners
...
```

#### Options for `list`

##### `--format`, `-f`
Specify output format.

```bash
soma-player list --format json
soma-player list --format csv
soma-player list -f table
```

**Formats:**
- `table` - Human-readable table (default)
- `json` - JSON format for scripting
- `csv` - CSV format for spreadsheets
- `simple` - Simple list of channel IDs

##### `--filter`, `-F`
Filter channels by genre or keyword.

```bash
soma-player list --filter electronic
soma-player list --filter ambient
soma-player list -F jazz
```

##### `--sort`, `-s`
Sort channels by criteria.

```bash
soma-player list --sort listeners
soma-player list --sort name
soma-player list -s genre
```

**Sort Options:**
- `name` - Alphabetical by channel name
- `listeners` - By listener count (descending)
- `genre` - By genre, then name
- `id` - By channel ID

### `config` - Configuration Management

Manage application configuration.

#### `config show`
Display current configuration.

```bash
soma-player config show
```

**Output:**
```toml
last_channel_id = "groovesalad"
volume = 75
auto_start = false

[logging]
level = "info"
file_rotation = true
```

#### `config get`
Get a specific configuration value.

```bash
soma-player config get volume
soma-player config get logging.level
```

#### `config set`
Set a configuration value.

```bash
soma-player config set volume 80
soma-player config set auto_start true
soma-player config set logging.level debug
```

#### `config reset`
Reset configuration to defaults.

```bash
# Reset all settings
soma-player config reset

# Reset specific section
soma-player config reset logging

# Reset specific key
soma-player config reset volume
```

#### `config validate`
Validate configuration file syntax and values.

```bash
soma-player config validate
```

**Output on success:**
```
Configuration is valid ✓
```

**Output on errors:**
```
Configuration errors:
- Invalid volume value '150' (must be 0-100)
- Unknown color_scheme 'rainbow'
```

#### `config backup`
Create a backup of the current configuration.

```bash
soma-player config backup
soma-player config backup --file /path/to/backup.toml
```

#### `config restore`
Restore configuration from a backup.

```bash
soma-player config restore
soma-player config restore --file /path/to/backup.toml
```

## Environment Variables

### Logging Control

#### `RUST_LOG`
Override log level and filtering.

```bash
# Debug level for all modules
RUST_LOG=debug soma-player

# Specific module debugging
RUST_LOG=soma_player::audio=debug soma-player

# Multiple modules
RUST_LOG=soma_player::audio=debug,soma_player::ui=info soma-player
```

#### `SOMA_LOG_FILE`
Custom log file location.

```bash
SOMA_LOG_FILE=/tmp/soma-debug.log soma-player
```

### Configuration Override

#### `SOMA_CONFIG_DIR`
Custom configuration directory.

```bash
SOMA_CONFIG_DIR=~/.soma soma-player
```

#### `SOMA_CONFIG_FILE`
Custom configuration file.

```bash
SOMA_CONFIG_FILE=/etc/soma-player.toml soma-player
```

### Audio Settings

#### `SOMA_AUDIO_BACKEND`
Force specific audio backend.

```bash
# Linux: Force ALSA
SOMA_AUDIO_BACKEND=alsa soma-player

# macOS: Force Core Audio
SOMA_AUDIO_BACKEND=coreaudio soma-player
```

#### `SOMA_SAMPLE_RATE`
Override audio sample rate.

```bash
SOMA_SAMPLE_RATE=48000 soma-player
```

### Network Settings

#### `SOMA_PROXY`
HTTP proxy for all requests.

```bash
SOMA_PROXY=http://proxy.example.com:8080 soma-player
```

#### `SOMA_TIMEOUT`
Network timeout in seconds.

```bash
SOMA_TIMEOUT=60 soma-player
```

## Exit Codes

### Standard Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Normal exit |
| 1 | General Error | Unspecified error |
| 2 | Argument Error | Invalid command-line arguments |
| 3 | Config Error | Configuration file issues |
| 4 | Network Error | Network connectivity problems |
| 5 | Audio Error | Audio system issues |
| 130 | Interrupted | Ctrl+C or SIGINT received |

### Examples
```bash
# Check exit code
soma-player play invalid_channel
echo $?  # Outputs: 4 (network error - channel not found)

# Use in scripts
if soma-player list --quiet; then
    echo "Channels available"
else
    echo "Failed to fetch channels"
fi
```

## Scripting and Automation

### Background Playback
```bash
# Start playing in background
nohup soma-player play groovesalad --no-ui > /dev/null 2>&1 &

# Save the process ID
echo $! > soma-player.pid

# Stop later
kill $(cat soma-player.pid)
```

### System Service Integration
```bash
# systemd service example
[Unit]
Description=SomaFM Player
After=network.target

[Service]
Type=simple
User=music
ExecStart=/usr/local/bin/soma-player play groovesalad --no-ui
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Scheduled Playback
```bash
# cron job to play during work hours
# Play Mon-Fri 9 AM to 5 PM
0 9 * * 1-5 /usr/local/bin/soma-player play groovesalad --duration 8h --no-ui
```

### Integration with Other Tools

#### i3/Polybar Integration
```bash
# Get current playing status
soma-player status --format=json | jq -r '.channel.title'

# Volume control
soma-player volume +5
soma-player volume -5
```

#### dmenu/rofi Channel Selection
```bash
# Channel selection menu
channel=$(soma-player list --format=simple | dmenu -p "Channel:")
if [ -n "$channel" ]; then
    soma-player play "$channel"
fi
```

## Troubleshooting Command-Line Issues

### Common Problems

#### Command Not Found
```bash
# Check if installed
which soma-player

# Check PATH
echo $PATH

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
```

#### Permission Denied
```bash
# Check file permissions
ls -la $(which soma-player)

# Make executable if needed
chmod +x ~/.local/bin/soma-player
```

#### Configuration Errors
```bash
# Validate configuration
soma-player config validate

# Reset if corrupted
soma-player config reset

# Use custom config temporarily
soma-player --config /dev/null
```

#### Audio Issues
```bash
# Test audio system
soma-player play groovesalad --log-level debug

# Try different backend
SOMA_AUDIO_BACKEND=pulse soma-player

# Check audio permissions
groups $USER | grep -E "(audio|pulse-access)"
```

### Debug Mode
```bash
# Full debug information
RUST_LOG=debug soma-player --verbose

# Save debug log
RUST_LOG=debug soma-player --verbose 2> debug.log
```

## Integration Examples

### Shell Functions
```bash
# Add to ~/.bashrc or ~/.zshrc

# Quick channel switching
soma() {
    if [ $# -eq 0 ]; then
        soma-player
    else
        soma-player play "$1"
    fi
}

# Channel search
soma-search() {
    soma-player list --filter "$1"
}

# Current status
soma-status() {
    soma-player status --format=simple
}
```

### Fish Shell Completions
```fish
# ~/.config/fish/completions/soma-player.fish
complete -c soma-player -n '__fish_use_subcommand' -a 'play list config help'
complete -c soma-player -n '__fish_seen_subcommand_from play' -a '(soma-player list --format=simple)'
```

### Zsh Completions
```zsh
# Add to ~/.zshrc
compdef '_files -W ~/.config/soma-player' soma-player --config
```

This comprehensive command-line reference covers all aspects of using SomaFM Player from the terminal and in automated scenarios.
