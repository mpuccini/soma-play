use std::fs;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use tracing_appender::non_blocking::WorkerGuard;

/// Configuration for the logging system
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: String,
    pub log_to_file: bool,
    pub log_to_console: bool,
    pub max_log_files: usize,
    pub max_file_size: u64,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_to_file: true,
            log_to_console: false, // Disabled by default to avoid TUI interference
            max_log_files: 5,
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Initialize the logging system
/// Returns a guard that must be kept alive for the duration of the program
pub fn init_logging(config: LogConfig) -> Result<Option<WorkerGuard>, Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let mut layers = Vec::new();
    let mut guard = None;

    // File logging
    if config.log_to_file {
        let log_dir = get_log_directory()?;
        
        // Clean old log files
        clean_old_logs(&log_dir, config.max_log_files)?;
        
        let file_appender = tracing_appender::rolling::daily(&log_dir, "soma-player.log");
        let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
        guard = Some(file_guard);

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false); // No colors in file logs
        
        layers.push(file_layer.boxed());
    }

    // Console logging (usually disabled for TUI apps)
    if config.log_to_console {
        let console_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_ansi(true); // Colors for console
        
        layers.push(console_layer.boxed());
    }

    // Initialize the subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(layers)
        .init();

    tracing::info!("Logging initialized with level: {}", config.level);
    
    Ok(guard)
}

/// Get the log directory path
fn get_log_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let log_dir = home.join(".config").join("soma-player").join("logs");
    
    // Create log directory if it doesn't exist
    fs::create_dir_all(&log_dir)?;
    
    Ok(log_dir)
}

/// Clean old log files to maintain the specified limit
fn clean_old_logs(log_dir: &PathBuf, max_files: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut log_files: Vec<_> = fs::read_dir(log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| ext == "log")
        })
        .collect();

    // Sort by modification time (newest first)
    log_files.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
        let b_time = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
        b_time.cmp(&a_time)
    });

    // Remove excess files
    if log_files.len() > max_files {
        for file in log_files.iter().skip(max_files) {
            if let Err(e) = fs::remove_file(file.path()) {
                eprintln!("Warning: Failed to remove old log file {:?}: {}", file.path(), e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, "info");
        assert!(config.log_to_file);
        assert!(!config.log_to_console);
    }

    #[test]
    fn test_clean_old_logs() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().to_path_buf();

        // Create some test log files
        for i in 0..7 {
            let file_path = log_dir.join(format!("test-{}.log", i));
            fs::write(&file_path, "test log content").unwrap();
        }

        // Clean logs, keeping only 3
        clean_old_logs(&log_dir, 3).unwrap();

        // Count remaining files
        let remaining = fs::read_dir(&log_dir).unwrap().count();
        assert_eq!(remaining, 3);
    }
}
