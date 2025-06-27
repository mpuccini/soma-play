//! # SomaFM Player
//!
//! A terminal-based music player for SomaFM internet radio stations.
//!
//! This crate provides a complete TUI application for streaming SomaFM radio stations
//! with features like volume control, channel selection, and persistent configuration.
//!
//! ## Modules
//!
//! - [`config`] - Configuration management and persistent settings
//! - [`ui`] - Terminal user interface components and rendering
//! - [`audio`] - Audio playback engine and stream handling
//! - [`api`] - SomaFM API integration and playlist parsing
//! - [`models`] - Data structures and type definitions
//! - [`logging`] - Logging configuration and management
//! - [`errors`] - Error types and handling utilities
//!
//! ## Example
//!
//! ```rust,no_run
//! use soma_player::{
//!     config::AppConfig,
//!     logging::{LogConfig, init_logging},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize logging
//!     let _guard = init_logging(LogConfig::default())?;
//!     
//!     // Load configuration
//!     let config = AppConfig::load()?;
//!     
//!     // Start the application
//!     // ... (main application logic)
//!     
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod ui;
pub mod audio;
pub mod api;
pub mod models;
pub mod logging;
pub mod errors;

pub use models::*;
pub use errors::*;
