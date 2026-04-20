//! About screen implementation
//!
//! Information about the application.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// About screen renderer
pub struct AboutScreen;

impl AboutScreen {
    pub fn draw(f: &mut Frame, _app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(f.area());

        let version = env!("CARGO_PKG_VERSION");

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "BloatwareHatao",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("Version {}", version),
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "The ultimate Android bloatware removal tool",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Author: ImKKingshuk",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "License: MIT",
                Style::default().fg(Color::Green),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "https://github.com/ImKKingshuk/BloatwareHatao",
                Style::default().fg(Color::Blue),
            )),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "Thank you for using BloatwareHatao!",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        let content_block = Block::default()
            .borders(Borders::ALL)
            .title(" ℹ️ About ")
            .style(Style::default().fg(Color::White));

        let paragraph = Paragraph::new(lines)
            .block(content_block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[0]);

        // Footer
        let footer = Paragraph::new(Line::from(vec![
            Span::raw("Press "),
            Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to return to Main Menu"),
        ]))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        f.render_widget(footer, chunks[1]);
    }
}
