//! Configuration management for the SomaFM Player.
//!
//! This module provides functionality for loading, saving, and managing
//! application configuration using TOML format. Configuration is stored
//! in the user's config directory (`~/.config/soma-player/config.toml`).
//!
//! # Examples
//!
//! ```rust
//! use soma_player::config::AppConfig;
//!
//! // Load configuration (creates default if not exists)
//! let mut config = AppConfig::load().unwrap();
//!
//! // Update settings
//! config.set_volume(75).unwrap();
//! config.set_last_channel("groovesalad".to_string()).unwrap();
//! config.set_auto_start(true).unwrap();
//! ```

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure.
///  
/// Stores user preferences and settings that persist between application runs.
/// Configuration is automatically saved to `~/.config/soma-player/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// ID of the last played channel (auto-saved when switching channels)
    pub last_channel_id: Option<String>,
    /// Volume level (0-100), defaults to 50
    pub volume: Option<u8>,
    /// Whether to automatically start playing the last channel on startup
    pub auto_start: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            last_channel_id: None,
            volume: Some(50),
            auto_start: false,
        }
    }
}

impl AppConfig {
    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let config_dir = home.join(".config").join("soma-player");
        
        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;
        
        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(config_path)?;
            let config: AppConfig = toml::from_str(&contents)?;
            Ok(config)
        } else {
            // Create default config and save it
            let default_config = Self::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }

    /// Update last used channel and save
    pub fn set_last_channel(&mut self, channel_id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.last_channel_id = Some(channel_id);
        self.save()
    }

    /// Update volume setting and save
    pub fn set_volume(&mut self, volume: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.volume = Some(volume.clamp(0, 100));
        self.save()
    }

    /// Update auto_start setting and save
    pub fn set_auto_start(&mut self, auto_start: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.auto_start = auto_start;
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        
        assert_eq!(config.last_channel_id, None);
        assert_eq!(config.volume, Some(50));
        assert_eq!(config.auto_start, false);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            last_channel_id: Some("groovesalad".to_string()),
            volume: Some(75),
            auto_start: true,
        };

        let toml_string = toml::to_string(&config).unwrap();
        let deserialized: AppConfig = toml::from_str(&toml_string).unwrap();

        assert_eq!(config.last_channel_id, deserialized.last_channel_id);
        assert_eq!(config.volume, deserialized.volume);
        assert_eq!(config.auto_start, deserialized.auto_start);
    }

    #[test]
    fn test_set_volume_clamps_values() {
        let mut config = AppConfig::default();
        
        // Test normal value
        config.set_volume(75).unwrap();
        assert_eq!(config.volume, Some(75));
        
        // Test clamping max value
        config.set_volume(150).unwrap();
        assert_eq!(config.volume, Some(100));
        
        // Test minimum value (should work with saturating_sub)
        config.set_volume(0).unwrap();
        assert_eq!(config.volume, Some(0));
    }

    #[test]
    fn test_set_last_channel() {
        let mut config = AppConfig::default();
        
        config.set_last_channel("deepspaceone".to_string()).unwrap();
        assert_eq!(config.last_channel_id, Some("deepspaceone".to_string()));
    }

    #[test]
    fn test_set_auto_start() {
        let mut config = AppConfig::default();
        
        config.set_auto_start(true).unwrap();
        assert_eq!(config.auto_start, true);
        
        config.set_auto_start(false).unwrap();
        assert_eq!(config.auto_start, false);
    }

    // Integration test for save/load cycle
    #[test]
    fn test_save_load_cycle() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create and save a config
        let original_config = AppConfig {
            last_channel_id: Some("spacestation".to_string()),
            volume: Some(80),
            auto_start: true,
        };

        // Write manually to test file
        let toml_content = toml::to_string_pretty(&original_config).unwrap();
        fs::write(&config_path, toml_content).unwrap();

        // Read and verify
        let loaded_content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: AppConfig = toml::from_str(&loaded_content).unwrap();

        assert_eq!(original_config.last_channel_id, loaded_config.last_channel_id);
        assert_eq!(original_config.volume, loaded_config.volume);
        assert_eq!(original_config.auto_start, loaded_config.auto_start);
    }
}
