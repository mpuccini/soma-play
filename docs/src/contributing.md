# Contributing

Welcome to SomaFM Player! We're excited to have you contribute to this project.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **System Dependencies** (Linux only)
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libasound2-dev pkg-config
   
   # Fedora/CentOS/RHEL
   sudo dnf install alsa-lib-devel pkg-config
   
   # Arch Linux
   sudo pacman -S alsa-lib pkg-config
   ```

3. **Git**
   ```bash
   git --version
   ```

### Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork**
   ```bash
   git clone https://github.com/yourusername/soma-play.git
   cd soma-play
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/mpuccini/soma-play.git
   ```

4. **Install development tools**
   ```bash
   # Code formatting
   rustup component add rustfmt
   
   # Linting
   rustup component add clippy
   
   # Documentation
   cargo install mdbook
   
   # Security auditing
   cargo install cargo-audit
   
   # Test coverage
   cargo install cargo-tarpaulin
   ```

5. **Build and test**
   ```bash
   cargo build
   cargo test
   cargo run
   ```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

**Branch Naming Conventions:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 2. Make Your Changes

Follow our coding standards and guidelines (see below).

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test config::tests

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Generate test coverage
cargo tarpaulin --out Html
```

### 4. Commit Your Changes

**Commit Message Format:**
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Code style changes
- `refactor` - Code refactoring
- `test` - Test additions/improvements
- `chore` - Maintenance tasks

**Examples:**
```bash
git commit -m "feat(audio): add volume fade transitions"
git commit -m "fix(ui): resolve channel list scrolling issue"
git commit -m "docs(api): update configuration examples"
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Coding Standards

### Rust Code Style

1. **Use `cargo fmt`** for consistent formatting
2. **Follow Rust naming conventions**
   - `snake_case` for functions and variables
   - `PascalCase` for types and structs
   - `SCREAMING_SNAKE_CASE` for constants

3. **Add documentation comments**
   ```rust
   /// Plays audio from the specified stream URL.
   ///
   /// # Arguments
   /// * `url` - The stream URL to play from
   ///
   /// # Examples
   /// ```
   /// let mut player = AudioPlayer::new()?;
   /// player.play("http://stream.somafm.com/groovesalad".to_string())?;
   /// ```
   pub fn play(&mut self, url: String) -> Result<()> {
       // Implementation
   }
   ```

4. **Handle errors properly**
   ```rust
   // Good: Use Result types and proper error handling
   fn load_config() -> Result<Config, ConfigError> {
       let content = fs::read_to_string(&path)
           .map_err(ConfigError::FileRead)?;
       toml::from_str(&content)
           .map_err(ConfigError::Parse)
   }
   
   // Avoid: Using unwrap() or expect() in library code
   ```

5. **Use meaningful variable names**
   ```rust
   // Good
   let selected_channel_index = 0;
   let stream_url = "http://...";
   
   // Avoid
   let i = 0;
   let url = "http://...";
   ```

### Module Organization

1. **Keep modules focused** - Each module should have a single responsibility
2. **Use clear imports** - Prefer explicit imports over glob imports
3. **Organize by feature** - Group related functionality together

```rust
// Good: Explicit imports
use crate::config::Config;
use crate::models::{Channel, PlayerState};

// Avoid: Glob imports (in most cases)
use crate::models::*;
```

### Error Handling

1. **Use custom error types** with `thiserror`
2. **Provide context** for errors
3. **Convert errors at module boundaries**

```rust
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Failed to initialize audio device: {0}")]
    DeviceInit(String),
    
    #[error("Stream error: {0}")]
    Stream(#[from] StreamError),
    
    #[error("Invalid volume level: {level} (must be 0-100)")]
    InvalidVolume { level: u8 },
}
```

### Testing Guidelines

1. **Write unit tests** for all public functions
2. **Use integration tests** for complex workflows
3. **Mock external dependencies** in tests
4. **Test error conditions** as well as success cases

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_save_and_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        
        let original_config = Config {
            volume: 75,
            last_channel_id: Some("groovesalad".to_string()),
            auto_start: true,
        };
        
        // Test save
        original_config.save_to_path(&config_path).unwrap();
        
        // Test load
        let loaded_config = Config::load_from_path(&config_path).unwrap();
        
        assert_eq!(original_config.volume, loaded_config.volume);
        assert_eq!(original_config.last_channel_id, loaded_config.last_channel_id);
        assert_eq!(original_config.auto_start, loaded_config.auto_start);
    }
    
    #[test]
    fn test_invalid_volume_returns_error() {
        let mut player = AudioPlayer::new().unwrap();
        let result = player.set_volume(150); // Invalid volume > 100
        assert!(result.is_err());
    }
}
```

## Documentation

### Code Documentation

1. **Document all public APIs** with examples
2. **Include usage examples** in documentation
3. **Document error conditions** and return types
4. **Keep documentation up to date** with code changes

### User Documentation

1. **Update relevant guide sections** when adding features
2. **Include configuration examples** for new settings
3. **Add troubleshooting entries** for known issues
4. **Update the changelog** with user-facing changes

### Building Documentation

```bash
# Generate API documentation
cargo doc --open

# Build user guide
cd docs && mdbook serve --open

# Build all documentation
./build-docs.sh
```

## Feature Development

### Adding New Features

1. **Create an issue** describing the feature
2. **Discuss the approach** with maintainers
3. **Start with tests** (TDD approach when possible)
4. **Implement incrementally** with small, focused commits
5. **Update documentation** before marking as complete

### Feature Guidelines

1. **Maintain backward compatibility** when possible
2. **Add configuration options** for user preferences
3. **Consider performance impact** of new features
4. **Ensure cross-platform compatibility**

### Example: Adding a New Feature

```rust
// 1. Define the feature interface
pub trait Equalizer {
    fn set_bands(&mut self, bands: Vec<f32>) -> Result<()>;
    fn get_bands(&self) -> Vec<f32>;
}

// 2. Add configuration support
#[derive(Serialize, Deserialize)]
pub struct EqualizerConfig {
    pub enabled: bool,
    pub bands: Vec<f32>,
}

// 3. Implement the feature
pub struct GraphicEqualizer {
    bands: Vec<f32>,
    enabled: bool,
}

// 4. Add tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_equalizer_bands() {
        let mut eq = GraphicEqualizer::new();
        let bands = vec![0.0, 2.0, 4.0, 2.0, 0.0];
        eq.set_bands(bands.clone()).unwrap();
        assert_eq!(eq.get_bands(), bands);
    }
}

// 5. Update UI integration
impl App {
    pub fn toggle_equalizer(&mut self) -> Result<()> {
        // Implementation
    }
}
```

## Bug Fixes

### Finding and Reporting Bugs

1. **Search existing issues** before creating new ones
2. **Provide detailed reproduction steps**
3. **Include system information** (OS, Rust version, etc.)
4. **Add logs or error messages** when applicable

### Fixing Bugs

1. **Write a test** that reproduces the bug
2. **Fix the issue** with minimal changes
3. **Verify the test passes** with your fix
4. **Check for similar issues** in related code

## Performance Considerations

### Optimization Guidelines

1. **Profile before optimizing** - Measure, don't guess
2. **Focus on hot paths** - Audio playback, UI rendering
3. **Consider memory usage** - Avoid unnecessary allocations
4. **Benchmark changes** - Ensure improvements are real

### Performance Testing

```bash
# Run benchmarks (if available)
cargo bench

# Profile memory usage
cargo run --release -- --log-level=debug

# Monitor resource usage during development
htop  # or similar system monitor
```

## Security

### Security Guidelines

1. **Validate all inputs** from external sources
2. **Use secure HTTP** for API communications
3. **Handle credentials safely** (if any)
4. **Audit dependencies** regularly

```bash
# Run security audit
cargo audit

# Check for known vulnerabilities
cargo audit --database advisory
```

## Release Process

### Preparing a Release

1. **Update version numbers** in `Cargo.toml`
2. **Update CHANGELOG.md** with new features and fixes
3. **Run full test suite** with `cargo test`
4. **Generate and review documentation**
5. **Test on target platforms**

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] Changelog is updated
- [ ] Version numbers are bumped
- [ ] No security vulnerabilities
- [ ] Performance regressions checked
- [ ] Cross-platform compatibility verified

## Community Guidelines

### Communication

1. **Be respectful** and inclusive
2. **Ask questions** when you're unsure
3. **Provide constructive feedback** in reviews
4. **Help newcomers** get started

### Code Reviews

1. **Review for correctness** and style
2. **Suggest improvements** rather than just pointing out problems
3. **Acknowledge good work** and clever solutions
4. **Be open to feedback** on your own code

### Issue Management

1. **Provide clear descriptions** for issues and PRs
2. **Link related issues** and PRs
3. **Update status** as work progresses
4. **Close issues** when resolved

## Getting Help

### Resources

- **Documentation**: [User Guide](../introduction.md) and [API Reference](../api.md)
- **Source Code**: Well-commented and documented
- **Issues**: Check existing issues for similar problems
- **Discussions**: GitHub Discussions for questions and ideas

### Contact

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Requests**: For code contributions and reviews

Thank you for contributing to SomaFM Player! Your efforts help make this project better for everyone. 🎵
