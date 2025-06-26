use crossterm::event::{KeyCode, KeyEvent};
use crate::ui::app::{AppState, UIState};
use crate::config::AppConfig;
use crate::audio::PlayerCommand;
use log::{error, info};

pub enum EventResult {
    ChannelChange(usize),
    PlayerCommand(PlayerCommand),
    Quit,
    None,
}

pub fn handle_key_event(
    app: &mut AppState,
    key: KeyEvent,
    channels_len: usize,
    current_channel_index: Option<usize>,
    config: &mut AppConfig
) -> EventResult {
    match (&app.ui_state, key.code) {
        // Initial channel selection
        (UIState::InitialChannelSelection, KeyCode::Up) => {
            app.previous_channel(channels_len);
            EventResult::None
        }
        (UIState::InitialChannelSelection, KeyCode::Down) => {
            app.next_channel(channels_len);
            EventResult::None
        }
        (UIState::InitialChannelSelection, KeyCode::Enter) => {
            if app.selected_index < channels_len {
                EventResult::ChannelChange(app.selected_index)
            } else {
                EventResult::None
            }
        }
        (UIState::InitialChannelSelection, KeyCode::Char('q') | KeyCode::Char('Q')) => {
            app.quit();
            EventResult::Quit
        }

        // Playing mode
        (UIState::Playing, KeyCode::Char('c') | KeyCode::Char('C')) => {
            app.set_channel_selection_mode(current_channel_index);
            EventResult::None
        }
        (UIState::Playing, KeyCode::Char('+') | KeyCode::Char('=')) => {
            // Increase volume
            if let Some(current_vol) = config.volume {
                let new_vol = (current_vol + 5).min(100);
                if let Err(e) = config.set_volume(new_vol) {
                    error!("Failed to save volume: {}", e);
                } else {
                    info!("Volume increased to {}%", new_vol);
                    return EventResult::PlayerCommand(PlayerCommand::SetVolume(new_vol));
                }
            }
            EventResult::None
        }
        (UIState::Playing, KeyCode::Char('-') | KeyCode::Char('_')) => {
            // Decrease volume
            if let Some(current_vol) = config.volume {
                let new_vol = current_vol.saturating_sub(5);
                if let Err(e) = config.set_volume(new_vol) {
                    error!("Failed to save volume: {}", e);
                } else {
                    info!("Volume decreased to {}%", new_vol);
                    return EventResult::PlayerCommand(PlayerCommand::SetVolume(new_vol));
                }
            }
            EventResult::None
        }
        (UIState::Playing, KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc) => {
            app.quit();
            EventResult::Quit
        }

        // Channel selection while playing
        (UIState::SelectingChannel, KeyCode::Up) => {
            app.previous_channel(channels_len);
            EventResult::None
        }
        (UIState::SelectingChannel, KeyCode::Down) => {
            app.next_channel(channels_len);
            EventResult::None
        }
        (UIState::SelectingChannel, KeyCode::Enter) => {
            if app.selected_index < channels_len {
                EventResult::ChannelChange(app.selected_index)
            } else {
                EventResult::None
            }
        }
        (UIState::SelectingChannel, KeyCode::Esc) => {
            app.set_playing_mode();
            EventResult::None
        }
        (UIState::SelectingChannel, KeyCode::Char('q') | KeyCode::Char('Q')) => {
            app.quit();
            EventResult::Quit
        }

        _ => EventResult::None,
    }
}
