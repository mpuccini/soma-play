use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub last_channel_id: Option<String>,
    pub volume: Option<u8>,
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
