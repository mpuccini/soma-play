use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::models::{Channel, TrackInfo};

/// Renders the initial channel selection UI
pub fn render_initial_channel_selection(
    frame: &mut Frame,
    channels: &[Channel],
    selected_index: usize
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Channel list
            Constraint::Length(3), // Controls
        ])
        .split(frame.area());

    // Header
    let header = ratatui::widgets::Paragraph::new("🎵 Welcome to SomaFM Player - Select a Channel 🎵")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(header, chunks[0]);

    // Channel list
    let items: Vec<ListItem> = channels
        .iter()
        .enumerate()
        .map(|(i, channel)| {
            let style = if i == selected_index {
                Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let content = format!("{:>3}. {} - {}", i + 1, channel.title, channel.description);
            ListItem::new(content).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    let channels_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Channels"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol("► ");
    
    frame.render_stateful_widget(channels_list, chunks[1], &mut list_state);

    // Controls
    let controls_text = vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigate  |  "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" - Select Channel  |  "),
            Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit"),
        ]),
    ];

    let controls_widget = ratatui::widgets::Paragraph::new(controls_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(controls_widget, chunks[2]);
}

/// Renders the channel selection UI while music continues playing
pub fn render_channel_selection(
    frame: &mut Frame,
    channels: &[Channel],
    current_channel: &Channel,
    track_info: &TrackInfo,
    selected_index: usize
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Current playing info
            Constraint::Min(8),    // Channel list
            Constraint::Length(3), // Controls
        ])
        .split(frame.area());

    // Header
    let header = ratatui::widgets::Paragraph::new("🎵 Select New Channel (Music Still Playing) 🎵")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(header, chunks[0]);

    // Current playing info
    let current_info = format!("🔊 Currently: {} - {} - {}", current_channel.title, track_info.artist, track_info.title);
    let current_widget = ratatui::widgets::Paragraph::new(current_info)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(current_widget, chunks[1]);

    // Channel list
    let items: Vec<ListItem> = channels
        .iter()
        .enumerate()
        .map(|(i, channel)| {
            let style = if i == selected_index {
                Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
            } else if channel.id == current_channel.id {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if channel.id == current_channel.id {
                "♪ "
            } else {
                "  "
            };

            let content = format!("{}{:>3}. {}", prefix, i + 1, channel.title);
            ListItem::new(content).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    let channels_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Channels"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol("► ");
    
    frame.render_stateful_widget(channels_list, chunks[2], &mut list_state);

    // Controls
    let controls_text = vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigate  |  "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" - Select  |  "),
            Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Cancel  |  "),
            Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit"),
        ]),
    ];

    let controls_widget = ratatui::widgets::Paragraph::new(controls_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(controls_widget, chunks[3]);
}
