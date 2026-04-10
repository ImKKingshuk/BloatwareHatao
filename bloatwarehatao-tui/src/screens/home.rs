//! Home screen implementation
//!
//! Main menu of the application.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::state::DeviceState;

/// Home screen renderer
pub struct HomeScreen;

impl HomeScreen {
    /// Draw the home screen
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),  // Header/banner
                Constraint::Min(10),    // Menu
                Constraint::Length(3),  // Status bar
            ])
            .split(f.area());

        Self::draw_banner(f, chunks[0]);
        Self::draw_menu(f, chunks[1], app);
        Self::draw_status_bar(f, chunks[2], app);
    }

    /// Draw the ASCII banner
    fn draw_banner(f: &mut Frame, area: Rect) {
        let banner_text = r#"
    ____  __            __                           __  __      __            
   / __ )/ /___  ____ _/ /__      ______ ________   / / / /___ _/ /_____ ___   
  / __  / / __ \/ __ `/ __/ | /| / / __ `/ ___/ _ \/ /_/ / __ `/ __/ __ `/ _ \  
 / /_/ / / /_/ / /_/ / /_ | |/ |/ / /_/ / /  /  __/ __  / /_/ / /_/ /_/ /  __/  
/_____/_/\____/\__,_/\__/ |__/|__/\__,_/_/   \___/_/ /_/\__,_/\__/\__,_/\___/   
                                                                               
"#;

        let banner = Paragraph::new(banner_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default());

        f.render_widget(banner, area);
    }

    /// Draw the main menu
    fn draw_menu(f: &mut Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        // Menu list
        let items: Vec<ListItem> = app
            .menu_items()
            .iter()
            .enumerate()
            .map(|(i, (title, _))| {
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == app.selected_index { "▶ " } else { "  " };
                ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(*title, style),
                ]))
            })
            .collect();

        let menu = List::new(items)
            .block(
                Block::default()
                    .title(" Main Menu ")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(menu, chunks[0]);

        // Info panel on the right
        Self::draw_info_panel(f, chunks[1], app);
    }

    /// Draw the info panel with device status and description
    fn draw_info_panel(f: &mut Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(5)])
            .split(area);

        // Device status card
        Self::draw_device_card(f, chunks[0], app);

        // Description panel
        let description = app
            .menu_items()
            .get(app.selected_index)
            .map(|(_, desc)| *desc)
            .unwrap_or("");

        let desc_block = Block::default()
            .title(" Description ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                description,
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to select",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "Press ? for help",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let desc = Paragraph::new(lines)
            .block(desc_block)
            .wrap(Wrap { trim: true });

        f.render_widget(desc, chunks[1]);
    }

    /// Draw the device status card
    fn draw_device_card(f: &mut Frame, area: Rect, app: &App) {
        let (status_text, status_color) = match &app.state.device {
            DeviceState::Unknown => ("⏳ Checking...", Color::Yellow),
            DeviceState::Checking => ("🔍 Scanning...", Color::Yellow),
            DeviceState::NotConnected => ("❌ No device", Color::Red),
            DeviceState::Unauthorized => ("🔒 Unauthorized", Color::Yellow),
            DeviceState::Connected(info) => {
                let text = format!("✅ {} {}", info.brand, info.model);
                return Self::draw_connected_device(f, area, &text, info);
            }
            DeviceState::Error(e) => {
                let text = format!("⚠️ {}", e);
                return Self::draw_device_status(f, area, &text, Color::Red);
            }
        };

        Self::draw_device_status(f, area, status_text, status_color);
    }

    fn draw_device_status(f: &mut Frame, area: Rect, text: &str, color: Color) {
        let block = Block::default()
            .title(" Device ")
            .title_style(Style::default().fg(Color::Magenta))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let paragraph = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(text, Style::default().fg(color))),
        ])
        .block(block)
        .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn draw_connected_device(f: &mut Frame, area: Rect, text: &str, info: &crate::state::DeviceInfo) {
        let block = Block::default()
            .title(" Device ")
            .title_style(Style::default().fg(Color::Magenta))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let paragraph = Paragraph::new(vec![
            Line::from(Span::styled(text, Style::default().fg(Color::Green))),
            Line::from(Span::styled(
                format!("Android {} | {}", info.android_version, info.oem),
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    /// Draw the status bar
    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let dry_run_indicator = if app.state.dry_run {
            Span::styled(" 🧪 DRY RUN ", Style::default().fg(Color::Black).bg(Color::Yellow))
        } else {
            Span::raw("")
        };

        let status_text = app
            .status_message
            .as_ref()
            .map(|s| Span::styled(format!(" {} ", s), Style::default().fg(Color::Green)))
            .unwrap_or_else(|| Span::styled(" Ready ", Style::default().fg(Color::DarkGray)));

        let help_text = Span::styled(
            " ↑/↓: Navigate | Enter: Select | q: Quit | ?: Help ",
            Style::default().fg(Color::DarkGray),
        );

        let status = Paragraph::new(Line::from(vec![dry_run_indicator, status_text, help_text]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(status, area);
    }
}
