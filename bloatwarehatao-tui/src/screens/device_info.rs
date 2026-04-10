//! Device info screen implementation
//!
//! Displays connected device information.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::state::DeviceState;

/// Device info screen renderer
pub struct DeviceInfoScreen;

impl DeviceInfoScreen {
    /// Draw the device info screen
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0]);
        Self::draw_content(f, chunks[1], app);
        Self::draw_status_bar(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect) {
        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                " 📱 Device Information ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(header, area);
    }

    fn draw_content(f: &mut Frame, area: Rect, app: &App) {
        let block = Block::default()
            .title(" Device Details ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let content = match &app.state.device {
            DeviceState::Unknown | DeviceState::Checking => {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "🔍 Checking for connected devices...",
                        Style::default().fg(Color::Yellow),
                    )),
                ])
                .block(block)
                .alignment(Alignment::Center)
            }
            DeviceState::NotConnected => {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "❌ No device connected",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        "Please connect an Android device with USB debugging enabled.",
                        Style::default().fg(Color::DarkGray),
                    )),
                    Line::from(""),
                    Line::from(Span::styled("Tips:", Style::default().fg(Color::Yellow))),
                    Line::from("  1. Enable Developer Options on your device"),
                    Line::from("  2. Enable USB Debugging in Developer Options"),
                    Line::from("  3. Connect your device via USB cable"),
                    Line::from("  4. Accept the USB debugging authorization prompt"),
                ])
                .block(block)
                .alignment(Alignment::Center)
            }
            DeviceState::Unauthorized => {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "🔒 Device Unauthorized",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        "Please check your device screen and accept the USB debugging prompt.",
                        Style::default().fg(Color::White),
                    )),
                ])
                .block(block)
                .alignment(Alignment::Center)
            }
            DeviceState::Connected(info) => {
                let lines = vec![
                    Line::from(""),
                    Self::info_line("Brand", &info.brand, Color::Cyan),
                    Self::info_line("Model", &info.model, Color::Cyan),
                    Self::info_line("Serial", &info.serial, Color::DarkGray),
                    Line::from(""),
                    Self::info_line("Android Version", &info.android_version, Color::Green),
                    Self::info_line("SDK Version", &info.sdk_version.to_string(), Color::Green),
                    Line::from(""),
                    Self::info_line("OEM Detected", &info.oem, Color::Yellow),
                ];

                Paragraph::new(lines)
                    .block(block)
                    .wrap(Wrap { trim: true })
            }
            DeviceState::Error(e) => {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "⚠️ Error",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(e.as_str(), Style::default().fg(Color::Red))),
                ])
                .block(block)
                .alignment(Alignment::Center)
            }
        };

        f.render_widget(content, area);
    }

    fn info_line(label: &str, value: &str, value_color: Color) -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("  {}: ", label), Style::default().fg(Color::DarkGray)),
            Span::styled(value.to_string(), Style::default().fg(value_color).add_modifier(Modifier::BOLD)),
        ])
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let help = Span::styled(
            " r: Refresh | ESC: Main Menu | q: Main Menu ",
            Style::default().fg(Color::DarkGray),
        );

        let dry_run = if app.state.dry_run {
            Span::styled(" 🧪 DRY RUN ", Style::default().fg(Color::Black).bg(Color::Yellow))
        } else {
            Span::raw("")
        };

        let status_bar = Paragraph::new(Line::from(vec![dry_run, help]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(status_bar, area);
    }
}
