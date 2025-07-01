use std::io;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use clap::Parser;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use soma_player::{
    api::fetch_channels,
    audio::{play_channel, PlayerCommand},
    config::AppConfig,
    models::{Channel, TrackInfo},
    ui::{
        app::{AppState, UIState},
        channel_list::{render_initial_channel_selection, render_channel_selection},
        player::render_playing_ui,
        events::{handle_key_event, EventResult},
    },
};

/// Terminal-based SomaFM radio player with spectrum visualizer
#[derive(Parser)]
#[command(author = "Marco Puccini <mpuccini@example.com>")]
#[command(version)]
#[command(about = "A terminal-based music player for SomaFM internet radio stations")]
#[command(long_about = "SomaFM Player streams high-quality internet radio from SomaFM stations. \
Features include a spectrum visualizer, channel browsing, persistent configuration, \
and keyboard controls for an optimal terminal experience.")]
#[command(after_help = "KEYBOARD CONTROLS:
    ↑/↓     Navigate channels
    Enter   Select channel  
    C       Change channel (while playing)
    P       Pause/Resume playback
    +/-     Volume control
    Q/Esc   Quit

EXAMPLES:
    soma-player                      Start with channel selection
    soma-player -a                   Auto-start with last channel
    soma-player -c groovesalad       Play Groove Salad directly
    soma-player -c dronezone -v 75   Play Drone Zone at 75% volume
    soma-player --list               List all available channels")]
struct Args {
    /// Start playing immediately without showing channel selection
    #[arg(short, long, help = "Skip channel selection screen and start playing")]
    autostart: bool,
    
    /// Set initial volume (0-100)
    #[arg(short, long, value_name = "LEVEL", help = "Set volume level (0-100)")]
    #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
    volume: Option<u8>,
    
    /// Play specific channel by ID (e.g., 'groovesalad', 'dronezone')
    #[arg(short, long, value_name = "CHANNEL_ID")]
    #[arg(help = "Play specific channel by ID (use --list to see available channels)")]
    channel: Option<String>,
    
    /// List all available channels and exit
    #[arg(short, long, help = "Display all available SomaFM channels and exit")]
    list: bool,
    
    /// Show config file location and exit
    #[arg(long, help = "Display configuration file path and exit")]
    config: bool,
}

async fn play_session_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    channels: &[Channel],
    selected_channel: &Channel,
    track_info: Arc<Mutex<TrackInfo>>,
    mut app: AppState,
    config: &mut AppConfig,
) -> Result<Option<usize>, String> {
    let (tx, rx) = mpsc::unbounded_channel();
    
    // Only start audio if we're not in initial selection mode
    let (audio_result_tx, mut audio_result_rx) = mpsc::unbounded_channel();
    let audio_handle = if !matches!(app.ui_state, UIState::InitialChannelSelection) {
        Some(tokio::task::spawn_blocking({
            let selected_channel = selected_channel.clone();
            let track_info = Arc::clone(&track_info);
            let audio_result_tx = audio_result_tx.clone();
            let volume = config.volume;
            move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(play_channel(&selected_channel, track_info, rx, volume));
                let _ = audio_result_tx.send(result);
            }
        }))
    } else {
        None
    };
    
    let mut last_update = std::time::Instant::now();
    
    
    loop {
        // Update display
        if last_update.elapsed() >= std::time::Duration::from_millis(100) {
            // Update spectrum visualizer based on current state
            let is_playing = matches!(app.ui_state, UIState::Playing | UIState::SelectingChannel);
            app.spectrum.update(is_playing, app.is_paused);
            
            let track = track_info.lock().await;
            if let Err(e) = terminal.draw(|frame| {
                match app.ui_state {
                    UIState::InitialChannelSelection => {
                        render_initial_channel_selection(frame, channels, app.selected_index)
                    }
                    UIState::Playing => {
                        render_playing_ui(frame, selected_channel, &track, config, &app)
                    }
                    UIState::SelectingChannel => {
                        render_channel_selection(frame, channels, selected_channel, &track, app.selected_index)
                    }
                }
            }) {
                break Err(format!("Failed to draw terminal: {}", e));
            }
            last_update = std::time::Instant::now();
        }
        
        // Handle keyboard input
        if let Ok(true) = event::poll(std::time::Duration::from_millis(10)) {
            if let Ok(Event::Key(key)) = event::read() {
                let current_channel_index = channels.iter().position(|c| c.id == selected_channel.id);
                
                match handle_key_event(
                    &mut app, 
                    key, 
                    channels.len(), 
                    current_channel_index,
                    config
                ) {
                    EventResult::ChannelChange(new_channel_index) => {
                        // Update config with selected channel
                        if let Some(channel) = channels.get(new_channel_index) {                        if let Err(e) = config.set_last_channel(channel.id.clone()) {
                            tracing::error!("Failed to save config: {}", e);
                        }
                        }
                        
                        // Send quit to current audio if playing
                        if audio_handle.is_some() {
                            let _ = tx.send(PlayerCommand::Quit);
                        }
                        break Ok(Some(new_channel_index));
                    }
                    EventResult::PlayerCommand(cmd) => {
                        // Send command to audio player
                        if audio_handle.is_some() {
                            let _ = tx.send(cmd);
                        }
                    }
                    EventResult::Quit => {
                        if audio_handle.is_some() {
                            let _ = tx.send(PlayerCommand::Quit);
                        }
                        break Ok(None);
                    }
                    EventResult::None => {
                        // Do nothing, continue loop
                    }
                }
                
                if app.should_quit {
                    if audio_handle.is_some() {
                        let _ = tx.send(PlayerCommand::Quit);
                    }
                    break Ok(None);
                }
            }
        }
        
        // Check audio task status
        if let Some(audio_handle) = &audio_handle {
            if let Ok(audio_result) = audio_result_rx.try_recv() {
                match audio_result {
                    Ok(_) => break Ok(None),
                    Err(e) => break Err(e),
                }
            }
            
            if audio_handle.is_finished() {
                break Ok(None);
            }
        }
        
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments - this automatically handles --help and --version
    let args = Args::parse();

    // Initialize enhanced logging system
    let _log_guard = soma_player::logging::init_logging(
        soma_player::logging::LogConfig::default()
    )?;

    color_eyre::install()?;
    tracing::info!("Starting SomaFM Player");
    
    // Handle special arguments that don't require the full app
    if args.config {
        match soma_player::config::AppConfig::config_path() {
            Ok(path) => {
                println!("Configuration file: {}", path.display());
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error getting config path: {}", e);
                return Err(e);
            }
        }
    }
    
    if args.list {
        println!("Fetching SomaFM channels...");
        match fetch_channels().await {
            Ok(channels) => {
                println!("\nAvailable channels:");
                for channel in channels {
                    println!("  {} - {}", channel.id, channel.title);
                    if !channel.description.is_empty() {
                        println!("    {}", channel.description);
                    }
                    println!();
                }
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error fetching channels: {}", e);
                return Err(e.into());
            }
        }
    }
    
    // Load configuration
    let mut config = soma_player::config::AppConfig::load().unwrap_or_default();
    tracing::debug!("Configuration loaded: {:?}", config);
    
    // Apply command-line overrides
    if args.autostart {
        config.auto_start = true;
    }
    
    if let Some(volume) = args.volume {
        if volume <= 100 {
            config.volume = Some(volume);
        } else {
            eprintln!("Warning: Volume must be between 0-100, ignoring value {}", volume);
        }
    }
    
    if let Some(channel_id) = args.channel {
        config.last_channel_id = Some(channel_id);
        config.auto_start = true; // Auto-start when specific channel is requested
    }
    
    let result = run_player(&mut config).await;
    
    if let Err(e) = &result {
        tracing::error!("Application error: {}", e);
    }
    
    result
}

async fn run_player(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let channels = fetch_channels().await?;
    let track_info = Arc::new(Mutex::new(TrackInfo::default()));
    
    // Try to find the last used channel or default to first
    let selected_channel_index = if let Some(ref last_id) = config.last_channel_id {
        channels.iter().position(|c| c.id == *last_id).unwrap_or(0)
    } else {
        0
    };
    
    let mut selected_channel = &channels[selected_channel_index];
    let mut first_run = true;
    
    loop {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let app = if first_run {
            if config.auto_start {
                // If auto_start is enabled, go directly to playing the last/first channel
                let mut app = AppState::new();
                app.ui_state = UIState::Playing;
                app
            } else {
                // Show channel selection screen
                AppState::new()
            }
        } else {
            let mut app = AppState::new();
            app.ui_state = UIState::Playing;
            app
        };

        let channel_selection = match play_session_tui(
            &mut terminal, 
            &channels, 
            selected_channel, 
            Arc::clone(&track_info), 
            app,
            config
        ).await {
            Ok(result) => {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                terminal.show_cursor()?;
                result
            }
            Err(e) => {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                terminal.show_cursor()?;
                return Err(e.into());
            }
        };
        
        match channel_selection {
            Some(index) => {
                if index < channels.len() {
                    selected_channel = &channels[index];
                    first_run = false;
                    tracing::info!("Switching to channel: {}", selected_channel.title);
                    
                    // Reset track info for new channel
                    {
                        let mut track = track_info.lock().await;
                        *track = TrackInfo::default();
                    }
                } else {
                    continue;
                }
            }
            None => {
                tracing::info!("User quit application");
                break;
            }
        }
    }
    
    tracing::info!("SomaFM Player shutting down");
    Ok(())
}


