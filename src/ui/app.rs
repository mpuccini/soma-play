use crate::models::AudioSpectrum;

#[derive(Debug, Clone)]
pub enum UIState {
    InitialChannelSelection,
    Playing,
    SelectingChannel,
}

pub struct AppState {
    pub ui_state: UIState,
    pub selected_index: usize,
    pub should_quit: bool,
    pub is_paused: bool,
    pub spectrum: AudioSpectrum,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ui_state: UIState::InitialChannelSelection,
            selected_index: 0,
            should_quit: false,
            is_paused: false,
            spectrum: AudioSpectrum::default(),
        }
    }

    pub fn next_channel(&mut self, max_channels: usize) {
        self.selected_index = if self.selected_index < max_channels - 1 { 
            self.selected_index + 1 
        } else { 
            0 
        };
    }

    pub fn previous_channel(&mut self, max_channels: usize) {
        self.selected_index = if self.selected_index > 0 { 
            self.selected_index - 1 
        } else { 
            max_channels - 1 
        };
    }

    pub fn set_channel_selection_mode(&mut self, current_channel_index: Option<usize>) {
        self.ui_state = UIState::SelectingChannel;
        if let Some(index) = current_channel_index {
            self.selected_index = index;
        }
    }

    pub fn set_playing_mode(&mut self) {
        self.ui_state = UIState::Playing;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }
}
