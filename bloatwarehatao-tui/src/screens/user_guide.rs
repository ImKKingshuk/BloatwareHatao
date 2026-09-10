//! User Guide screen implementation
//!
//! Displays a comprehensive guide on how to use the TUI.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

/// User Guide screen renderer
pub struct UserGuideScreen;

impl UserGuideScreen {
    pub fn draw(f: &mut Frame, app: &App) {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(size);

        // Header
        let header = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let title = Paragraph::new("📚 User Guide")
            .block(header)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);

        // Content
        let guide_text = vec![
            Line::from(Span::styled(
                "Welcome to BloatwareHatao TUI",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(
                "This tool helps you remove bloatware from your Android device safely and efficiently.",
            ),
            Line::from(""),
            Line::from(Span::styled(
                "1. Navigation",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• Use ↑/↓ Arrow keys to navigate menus and lists."),
            Line::from("• Press ENTER to select items or confirm actions."),
            Line::from("• Press ESC to go back to the previous screen or Main Menu."),
            Line::from("• Press 'q' to go back to the Main Menu."),
            Line::from(""),
            Line::from(Span::styled(
                "2. Connecting your Device",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• Enable 'USB Debugging' in your phone's Developer Options."),
            Line::from("• Connect via USB cable. The tool should detect it automatically."),
            Line::from("• For Wi-Fi: Go to 'Wireless ADB', enter pairing info from your phone."),
            Line::from(""),
            Line::from(Span::styled(
                "3. Package Browser",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• Browse all installed apps on your device."),
            Line::from("• Press SPACE to select apps for batch operations."),
            Line::from("• Press '/' to search by name or description."),
            Line::from("• Press 'i' to toggle System Apps visibility."),
            Line::from(""),
            Line::from(Span::styled(
                "4. Safety Ratings",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• 🟢 Recommended: Safe to remove, known bloatware."),
            Line::from("• 🟡 Advanced: Safe but may break some features."),
            Line::from("• 🟠 Expert: Remove only if you know what you are doing."),
            Line::from("• 🔴 Dangerous: Do NOT remove unless absolutely necessary (bootloops)."),
            Line::from("• 🔵 Safe: General safe apps (User Installed)."),
            Line::from(""),
            Line::from(Span::styled(
                "5. Profiles",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• Create profiles to save lists of apps to remove."),
            Line::from("• Useful for debloating multiple devices quickly."),
            Line::from(""),
            Line::from(Span::styled(
                "6. Backups",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )),
            Line::from("• Always backup before making changes!"),
            Line::from("• Use the 'Backup & Restore' menu to secure your data."),
            Line::from(""),
        ];

        let content_block = Block::default()
            .borders(Borders::ALL)
            .title(" Manual ")
            .style(Style::default().fg(Color::White));

        // In a real implementation with scrolling, we'd use app.state.user_guide.scroll_offset
        // For now, we wrap text.
        let content = Paragraph::new(guide_text)
            .block(content_block)
            .wrap(Wrap { trim: true })
            .scroll((app.state.user_guide.scroll_offset as u16, 0));

        f.render_widget(content, chunks[1]);

        // Footer
        let footer = Paragraph::new(Line::from(vec![
            Span::raw("Press "),
            Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to return to Main Menu | "),
            Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Scroll"),
        ]))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        f.render_widget(footer, chunks[2]);
    }
}

/// User Guide state
#[derive(Debug, Clone, Default)]
pub struct UserGuideState {
    pub scroll_offset: usize,
}
