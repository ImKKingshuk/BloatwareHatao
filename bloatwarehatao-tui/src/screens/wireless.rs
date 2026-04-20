//! Wireless ADB screen implementation
//!
//! Interface for managing wireless ADB connections.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// Wireless ADB screen renderer
pub struct WirelessScreen;

impl WirelessScreen {
    /// Draw the wireless screen
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Main content
                Constraint::Length(3), // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0]);
        Self::draw_content(f, chunks[1], app);
        Self::draw_status_bar(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let text = Paragraph::new(Span::styled(
            " 📡 Wireless ADB Connection ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .block(block)
        .alignment(Alignment::Center);

        f.render_widget(text, area);
    }

    fn draw_content(f: &mut Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Enable Wireless
                Constraint::Length(6), // Connect
                Constraint::Min(3),    // Status/Info
            ])
            .margin(1)
            .split(area);

        // Enable Wireless Section
        let enable_block = Block::default()
            .title(" Step 1: Enable Wireless on Device (USB Required) ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let port_text = vec![
            Line::from("Connect your device via USB first."),
            Line::from(""),
            Line::from(vec![
                Span::raw("Port: "),
                Span::styled(
                    &app.state.wireless.port_input,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" (Press 'p' to edit)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press 'e' to Enable Wireless Mode",
                Style::default().fg(Color::Green),
            )),
        ];

        f.render_widget(Paragraph::new(port_text).block(enable_block), chunks[0]);

        // Connect Section
        let connect_block = Block::default()
            .title(" Step 2: Connect via Wi-Fi ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let addr_text = vec![
            Line::from("Enter the IP address and port (e.g., 192.0.2.5:5555)"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Address: "),
                Span::styled(
                    if app.state.wireless.address_input.is_empty() {
                        "Enter address..."
                    } else {
                        &app.state.wireless.address_input
                    },
                    if app.state.wireless.address_input.is_empty() {
                        Style::default().fg(Color::DarkGray)
                    } else {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    },
                ),
                Span::styled(" (Press 'i' to edit)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press 'c' to Connect",
                Style::default().fg(Color::Green),
            )),
        ];

        f.render_widget(Paragraph::new(addr_text).block(connect_block), chunks[1]);

        // Status Section
        let status_block = Block::default()
            .title(" Status ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let status_msg = app.state.wireless.status.clone().unwrap_or_default();
        let status_style = if status_msg.starts_with("Error") || status_msg.starts_with("Failed") {
            Style::default().fg(Color::Red)
        } else if status_msg.starts_with("Success") || status_msg.starts_with("Connected") {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        f.render_widget(
            Paragraph::new(status_msg)
                .style(status_style)
                .block(status_block),
            chunks[2],
        );
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, _app: &App) {
        let help = Span::styled(
            " p: Edit Port | e: Enable | i: Edit Address | c: Connect | ESC: Main Menu | q: Main Menu ",
            Style::default().fg(Color::DarkGray),
        );

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        f.render_widget(Paragraph::new(help).block(block), area);
    }
}
