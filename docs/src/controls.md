# Controls Reference

Complete keyboard controls for SomaFM Player.

## Overview

SomaFM Player has three main interface modes, each with its own set of controls:
1. **Channel Selection Screen** - Initial startup screen
2. **Playing Mode** - Main playback interface
3. **Channel Selection Overlay** - Quick channel switching while playing

## Channel Selection Screen

This is the first screen you see when starting the application.

| Key | Action | Description |
|-----|--------|-------------|
| ↑ | Navigate Up | Move selection to previous channel |
| ↓ | Navigate Down | Move selection to next channel |
| Enter | Select Channel | Start playing the selected channel |
| Q | Quit | Exit the application |

### Mouse Support
- **Scroll Wheel**: Navigate through channels (if terminal supports it)
- **Click**: Select channel (if terminal supports it)

## Playing Mode

Once you've selected a channel and music is playing.

### Playback Controls
| Key | Action | Description |
|-----|--------|-------------|
| P | Pause/Resume | Toggle playback state |
| Q | Quit | Exit the application |
| Esc | Quit | Alternative quit key |

### Volume Controls
| Key | Action | Description |
|-----|--------|-------------|
| + | Volume Up | Increase volume by 5% |
| = | Volume Up | Alternative volume up key |
| - | Volume Down | Decrease volume by 5% |
| _ | Volume Down | Alternative volume down key |

**Volume Range**: 0% to 100%  
**Volume Step**: 5% per key press  
**Auto-save**: Volume changes are automatically saved

### Channel Controls
| Key | Action | Description |
|-----|--------|-------------|
| C | Change Channel | Open channel selection overlay |

## Channel Selection Overlay

Accessible by pressing **C** while in playing mode.

| Key | Action | Description |
|-----|--------|-------------|
| ↑ | Navigate Up | Move selection to previous channel |
| ↓ | Navigate Down | Move selection to next channel |
| Enter | Switch Channel | Change to selected channel and close overlay |
| Esc | Cancel | Close overlay without changing channel |
| Q | Quit | Exit the application |

### Behavior Notes
- **Seamless switching**: Audio continues playing while browsing
- **Instant change**: New channel starts immediately when selected
- **Previous state**: If cancelled, returns to current playing channel

## Special Key Combinations

### Force Quit
| Key Combination | Action | When to Use |
|----------------|--------|-------------|
| Ctrl+C | Force Quit | If application becomes unresponsive |
| Ctrl+Z | Suspend | Pause process (use `fg` to resume) |

⚠️ **Note**: Force quit may leave your terminal in an inconsistent state. Use normal quit (Q) when possible.

## Universal Controls

These keys work in all modes:

| Key | Available In | Action |
|-----|-------------|--------|
| Q | All modes | Quit application |
| Ctrl+C | All modes | Force quit |

## Key Behavior Details

### Navigation Keys
- **Continuous scrolling**: Hold arrow keys to scroll quickly
- **Wrap-around**: Navigation wraps from bottom to top and vice versa
- **Visual feedback**: Current selection is highlighted

### Volume Keys
- **Immediate effect**: Volume changes apply instantly
- **Visual feedback**: Volume level displayed in interface
- **Bounds checking**: Cannot go below 0% or above 100%
- **Persistence**: Settings saved automatically

### Response Time
- **Instant**: All key presses register immediately
- **No delay**: No artificial delays or key repeat issues
- **Reliable**: Input processing is prioritized

## Accessibility Features

### Keyboard-Only Operation
- **No mouse required**: Full functionality via keyboard
- **Clear navigation**: Obvious visual indicators for current selection
- **Consistent patterns**: Similar controls across all modes

### Visual Indicators
- **Highlighted selection**: Current item clearly marked
- **Status display**: Current channel, volume, and playback state visible
- **Color coding**: Different colors for different interface elements

## Terminal Compatibility

### Recommended Terminals
- ✅ **Alacritty**: Full compatibility
- ✅ **Kitty**: Full compatibility  
- ✅ **iTerm2** (macOS): Full compatibility
- ✅ **Windows Terminal**: Full compatibility
- ✅ **GNOME Terminal**: Full compatibility

### Key Support Notes
- **Function keys**: Not used (for maximum compatibility)
- **Alt combinations**: Not used (may conflict with terminal)
- **Standard keys only**: Uses only common, reliable key codes

## Customization

### Current Limitations
- **Fixed keybindings**: Keys cannot be customized currently
- **Future feature**: Customizable controls planned for future release

### Workarounds
- **Terminal keybinding**: Some terminals allow key remapping
- **Screen/tmux**: Can provide additional key handling if needed

## Quick Reference Card

### Essential Keys
```
Playing Mode:
  P = Pause/Resume    C = Change Channel
  + = Volume Up       - = Volume Down
  Q = Quit

Channel Selection:
  ↑↓ = Navigate      Enter = Select
  Q = Quit

Channel Overlay:
  ↑↓ = Navigate      Enter = Switch
  Esc = Cancel       Q = Quit
```

### Memory Tips
- **P**ause for **P**layback control
- **C**hange for **C**hannel selection
- **+/-** for volume (like calculator)
- **Q**uit to **Q**uit (universal)
- **Esc**ape to cancel overlay

This control scheme is designed to be intuitive and memorable while providing full functionality through the keyboard interface.
