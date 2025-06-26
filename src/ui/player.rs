use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::models::{Channel, TrackInfo};
use crate::config::AppConfig;

/// Renders the playing UI
pub fn render_playing_ui(frame: &mut Frame, channel: &Channel, track_info: &TrackInfo, config: &AppConfig) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(2), // Channel
            Constraint::Length(4), // Track info
            Constraint::Length(2), // Status
            Constraint::Min(0),    // Spacer
            Constraint::Length(3), // Controls
        ])
        .split(frame.area());

    // Header
    let header = ratatui::widgets::Paragraph::new("🎵 SomaFM Player 🎵")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(header, chunks[0]);

    // Channel info
    let channel_text = format!("📻 Channel: {}", channel.title);
    let channel_widget = ratatui::widgets::Paragraph::new(channel_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(channel_widget, chunks[1]);

    // Track info
    let artist_style = if track_info.artist != "Unknown" && track_info.artist != "Loading..." {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let title_style = if track_info.title != "Loading..." {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let track_text = vec![
        Line::from(vec![
            Span::styled("🎤 Artist: ", Style::default().fg(Color::Yellow)),
            Span::styled(&track_info.artist, artist_style),
        ]),
        Line::from(vec![
            Span::styled("🎵 Title:  ", Style::default().fg(Color::Yellow)),
            Span::styled(&track_info.title, title_style),
        ]),
    ];

    let track_widget = ratatui::widgets::Paragraph::new(track_text)
        .block(Block::default().borders(Borders::ALL).title("Now Playing"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(track_widget, chunks[2]);

    // Status with volume
    let volume_text = if let Some(vol) = config.volume {
        format!(" | 🔊 {}%", vol)
    } else {
        "".to_string()
    };
    
    let status_text = if track_info.title != "Loading..." {
        format!("🔊 Playing{}", volume_text)
    } else {
        format!("⏳ Connecting to {}...{}", channel.title, volume_text)
    };
    
    let status_color = if track_info.title != "Loading..." {
        Color::Green
    } else {
        Color::Yellow
    };

    let status_widget = ratatui::widgets::Paragraph::new(status_text)
        .style(Style::default().fg(status_color).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(status_widget, chunks[3]);

    // Controls
    let controls_text = vec![
        Line::from(vec![
            Span::styled("C", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Change channel  |  "),
            Span::styled("+/-", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" - Volume  |  "),
            Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit"),
        ]),
    ];

    let controls_widget = ratatui::widgets::Paragraph::new(controls_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(controls_widget, chunks[5]);
}
