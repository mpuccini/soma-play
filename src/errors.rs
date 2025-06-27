use std::fmt;

/// Custom error types for the SomaFM Player application
#[derive(Debug)]
pub enum PlayerError {
    /// Network-related errors
    Network(NetworkError),
    /// Audio-related errors
    Audio(AudioError),
    /// Configuration-related errors
    Config(ConfigError),
    /// UI-related errors
    UI(UIError),
    /// General I/O errors
    IO(std::io::Error),
    /// Parsing errors
    Parse(String),
}

#[derive(Debug)]
pub enum NetworkError {
    /// Failed to connect to SomaFM API
    ApiConnection(String),
    /// Failed to parse API response
    ApiParse(String),
    /// Failed to connect to stream
    StreamConnection(String),
    /// Invalid URL
    InvalidUrl(String),
    /// Network timeout
    Timeout,
}

#[derive(Debug)]
pub enum AudioError {
    /// Failed to initialize audio output
    OutputInit(String),
    /// Failed to create audio sink
    SinkCreation(String),
    /// Failed to decode audio stream
    DecodingError(String),
    /// Audio device not available
    DeviceUnavailable,
    /// Unsupported audio format
    UnsupportedFormat(String),
}

#[derive(Debug)]
pub enum ConfigError {
    /// Failed to create config directory
    DirectoryCreation(String),
    /// Failed to read config file
    FileRead(String),
    /// Failed to write config file
    FileWrite(String),
    /// Invalid configuration values
    InvalidValue(String),
    /// TOML parsing error
    TomlParse(String),
}

#[derive(Debug)]
pub enum UIError {
    /// Terminal setup failed
    TerminalInit(String),
    /// Failed to render UI
    RenderError(String),
    /// Event handling error
    EventError(String),
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerError::Network(e) => write!(f, "Network error: {}", e),
            PlayerError::Audio(e) => write!(f, "Audio error: {}", e),
            PlayerError::Config(e) => write!(f, "Configuration error: {}", e),
            PlayerError::UI(e) => write!(f, "UI error: {}", e),
            PlayerError::IO(e) => write!(f, "I/O error: {}", e),
            PlayerError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ApiConnection(msg) => write!(f, "Failed to connect to SomaFM API: {}", msg),
            NetworkError::ApiParse(msg) => write!(f, "Failed to parse API response: {}", msg),
            NetworkError::StreamConnection(msg) => write!(f, "Failed to connect to stream: {}", msg),
            NetworkError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            NetworkError::Timeout => write!(f, "Network request timed out"),
        }
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::OutputInit(msg) => write!(f, "Failed to initialize audio output: {}", msg),
            AudioError::SinkCreation(msg) => write!(f, "Failed to create audio sink: {}", msg),
            AudioError::DecodingError(msg) => write!(f, "Audio decoding error: {}", msg),
            AudioError::DeviceUnavailable => write!(f, "Audio device is not available"),
            AudioError::UnsupportedFormat(format) => write!(f, "Unsupported audio format: {}", format),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::DirectoryCreation(msg) => write!(f, "Failed to create config directory: {}", msg),
            ConfigError::FileRead(msg) => write!(f, "Failed to read config file: {}", msg),
            ConfigError::FileWrite(msg) => write!(f, "Failed to write config file: {}", msg),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::TomlParse(msg) => write!(f, "Failed to parse TOML: {}", msg),
        }
    }
}

impl fmt::Display for UIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIError::TerminalInit(msg) => write!(f, "Failed to initialize terminal: {}", msg),
            UIError::RenderError(msg) => write!(f, "Failed to render UI: {}", msg),
            UIError::EventError(msg) => write!(f, "Event handling error: {}", msg),
        }
    }
}

impl std::error::Error for PlayerError {}
impl std::error::Error for NetworkError {}
impl std::error::Error for AudioError {}
impl std::error::Error for ConfigError {}
impl std::error::Error for UIError {}

// Conversion implementations for common error types
impl From<std::io::Error> for PlayerError {
    fn from(error: std::io::Error) -> Self {
        PlayerError::IO(error)
    }
}

impl From<reqwest::Error> for PlayerError {
    fn from(error: reqwest::Error) -> Self {
        PlayerError::Network(NetworkError::ApiConnection(error.to_string()))
    }
}

impl From<toml::de::Error> for PlayerError {
    fn from(error: toml::de::Error) -> Self {
        PlayerError::Config(ConfigError::TomlParse(error.to_string()))
    }
}

impl From<toml::ser::Error> for PlayerError {
    fn from(error: toml::ser::Error) -> Self {
        PlayerError::Config(ConfigError::TomlParse(error.to_string()))
    }
}

/// Result type alias for the application
pub type PlayerResult<T> = Result<T, PlayerError>;

/// Helper trait for converting generic errors into PlayerError with context
pub trait PlayerErrorExt<T> {
    fn with_network_context(self, context: &str) -> PlayerResult<T>;
    fn with_audio_context(self, context: &str) -> PlayerResult<T>;
    fn with_config_context(self, context: &str) -> PlayerResult<T>;
    fn with_ui_context(self, context: &str) -> PlayerResult<T>;
}

impl<T, E: std::error::Error> PlayerErrorExt<T> for Result<T, E> {
    fn with_network_context(self, context: &str) -> PlayerResult<T> {
        self.map_err(|e| PlayerError::Network(NetworkError::ApiConnection(
            format!("{}: {}", context, e)
        )))
    }

    fn with_audio_context(self, context: &str) -> PlayerResult<T> {
        self.map_err(|e| PlayerError::Audio(AudioError::OutputInit(
            format!("{}: {}", context, e)
        )))
    }

    fn with_config_context(self, context: &str) -> PlayerResult<T> {
        self.map_err(|e| PlayerError::Config(ConfigError::FileRead(
            format!("{}: {}", context, e)
        )))
    }

    fn with_ui_context(self, context: &str) -> PlayerResult<T> {
        self.map_err(|e| PlayerError::UI(UIError::TerminalInit(
            format!("{}: {}", context, e)
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let network_err = PlayerError::Network(NetworkError::ApiConnection("test".to_string()));
        assert!(network_err.to_string().contains("Network error"));
        
        let audio_err = PlayerError::Audio(AudioError::DeviceUnavailable);
        assert!(audio_err.to_string().contains("Audio error"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let player_err: PlayerError = io_err.into();
        
        match player_err {
            PlayerError::IO(_) => {},
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_error_context() {
        let result: Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied, 
            "access denied"
        ));
        
        let player_result = result.with_config_context("Loading configuration");
        assert!(player_result.is_err());
        
        match player_result.unwrap_err() {
            PlayerError::Config(ConfigError::FileRead(msg)) => {
                assert!(msg.contains("Loading configuration"));
            },
            _ => panic!("Expected config error"),
        }
    }
}
