# Troubleshooting

Common issues and solutions for SomaFM Player.

## Installation Issues

### Binary Not Found
**Problem**: `soma-player: command not found`

**Solutions**:
1. **Check PATH**: Ensure `~/.local/bin` is in your PATH
   ```bash
   echo $PATH | grep -o ~/.local/bin
   ```

2. **Add to PATH** (if missing):
   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Verify installation**:
   ```bash
   ls -la ~/.local/bin/soma-player
   ```

### Permission Denied
**Problem**: `Permission denied` when running

**Solution**:
```bash
chmod +x ~/.local/bin/soma-player
```

### Audio Dependencies (Linux)
**Problem**: Audio playback fails on Linux

**Solutions**:
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora/CentOS/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

## Playback Issues

### No Audio Output
**Symptoms**: Player starts but no sound

**Debugging Steps**:
1. **Check system audio**:
   ```bash
   # Test system audio
   speaker-test -t sine -f 1000 -l 1
   ```

2. **Check volume levels**:
   - System volume is not muted
   - Player volume is above 0%

3. **Audio device selection**:
   - Ensure correct audio output device is selected
   - Try switching audio devices in system settings

### Stream Connection Fails
**Symptoms**: "Failed to connect to stream"

**Solutions**:
1. **Check internet connection**:
   ```bash
   ping somafm.com
   ```

2. **Try different channel**: Some channels may be temporarily unavailable

3. **Check firewall**: Ensure outbound HTTP connections are allowed

4. **Restart application**: Connection issues may be temporary

### Stuttering Audio
**Symptoms**: Audio cuts out or stutters

**Causes & Solutions**:
1. **Network issues**: Check internet stability
2. **High CPU usage**: Close other applications
3. **Audio buffer issues**: Restart the application
4. **System audio driver**: Update audio drivers

## Interface Issues

### Display Problems
**Problem**: Interface looks corrupted or misaligned

**Solutions**:
1. **Terminal size**: Ensure terminal is at least 80x24
   ```bash
   resize -s 24 80
   ```

2. **Terminal compatibility**: Use a modern terminal emulator
   - ✅ Recommended: Alacritty, Kitty, iTerm2, Windows Terminal
   - ⚠️ May have issues: Old terminals, basic consoles

3. **Color support**: Enable 256-color support
   ```bash
   echo $TERM  # Should show something like xterm-256color
   ```

### Spectrum Visualizer Issues
**Problem**: Visualizer not showing or appears static

**Solutions**:
1. **Terminal size**: Ensure sufficient space for visualizer
2. **Color support**: Enable color in terminal
3. **Refresh rate**: Try resizing terminal window

### Keyboard Input Not Working
**Problem**: Keys don't respond

**Solutions**:
1. **Terminal focus**: Ensure terminal window has focus
2. **Input capture**: Some terminals may capture certain keys
3. **Restart**: Try restarting the application

## Configuration Issues

### Settings Not Saving
**Problem**: Volume/channel changes don't persist

**Debugging**:
1. **Check permissions**:
   ```bash
   ls -la ~/.config/soma-player/
   ```

2. **Create directory** (if missing):
   ```bash
   mkdir -p ~/.config/soma-player
   ```

3. **Check disk space**:
   ```bash
   df -h ~/.config
   ```

### Configuration File Corrupted
**Problem**: Application fails to start with config error

**Solution**:
```bash
# Backup existing config
mv ~/.config/soma-player/config.toml ~/.config/soma-player/config.toml.backup

# Restart application (creates new default config)
soma-player
```

## Performance Issues

### High CPU Usage
**Symptoms**: System becomes slow while playing

**Solutions**:
1. **Check running processes**:
   ```bash
   top | grep soma-player
   ```

2. **Reduce terminal effects**: Use simpler terminal themes
3. **Update system**: Ensure OS and drivers are current

### Memory Usage
**Symptoms**: Memory usage grows over time

**Solutions**:
1. **Restart periodically**: Long-running sessions may accumulate memory
2. **Check logs**: Look for memory-related errors in logs
3. **Update**: Ensure you're running the latest version

## Network Issues

### Proxy/Corporate Network
**Problem**: Cannot connect through corporate proxy

**Solutions**:
1. **HTTP proxy**: Set proxy environment variables
   ```bash
   export http_proxy=http://proxy.company.com:8080
   export https_proxy=http://proxy.company.com:8080
   ```

2. **Direct connection**: Try from personal network to isolate issue

### Firewall Blocking
**Problem**: Connections blocked by firewall

**Required access**:
- **Outbound HTTP** (port 80)
- **Outbound HTTPS** (port 443)
- **Audio streams** (various ports)

## Logging and Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug soma-player
```

### Check Log Files
```bash
# View recent logs
tail -f ~/.config/soma-player/logs/soma-player.log

# Search for errors
grep -i error ~/.config/soma-player/logs/soma-player.log
```

### Collect Debug Information
For bug reports, collect:

1. **Version information**:
   ```bash
   soma-player --version
   ```

2. **System information**:
   ```bash
   uname -a
   ```

3. **Terminal information**:
   ```bash
   echo $TERM
   tput colors
   ```

4. **Log excerpt** (relevant error messages)

## Getting Help

### Before Reporting Issues
1. **Check this troubleshooting guide**
2. **Search existing issues** on GitHub
3. **Try with debug logging** enabled
4. **Test basic functionality** (can you run `soma-player --version`?)

### Reporting Bugs
Include this information:
- **Exact error message**
- **Steps to reproduce**
- **System information** (OS, terminal)
- **Log files** (with debug enabled)
- **Expected vs actual behavior**

### Community Support
- **GitHub Issues**: Technical problems and bugs
- **GitHub Discussions**: General questions and usage help

## Quick Fixes

### Complete Reset
If all else fails, completely reset the application:

```bash
# Stop the application
# Remove all application data
rm -rf ~/.config/soma-player

# Reinstall (if using install script)
curl -sSL https://raw.githubusercontent.com/mpuccini/soma-play/main/install.sh | bash

# Or rebuild from source
git clone https://github.com/mpuccini/soma-play.git
cd soma-play
cargo build --release
cp target/release/soma-player ~/.local/bin/
```

### Emergency Commands
If the interface becomes unresponsive:
- **Ctrl+C**: Force quit (may leave terminal in bad state)
- **Ctrl+Z**: Suspend (then `fg` to resume or `kill %1` to terminate)
- **Close terminal**: Last resort
