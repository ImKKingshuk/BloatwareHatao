//! Settings screen implementation
//!
//! Application configuration settings backed by bh-core Config.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use bloatwarehatao_core::config::Theme;

/// Settings categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    General,
    Removal,
    Appearance,
}

impl SettingsCategory {
    pub fn all() -> Vec<Self> {
        vec![Self::General, Self::Removal, Self::Appearance]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Removal => "Removal Options",
            Self::Appearance => "Appearance",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::General => "⚙️",
            Self::Removal => "🗑️",
            Self::Appearance => "🎨",
        }
    }
}

/// Settings screen renderer
pub struct SettingsScreen;

impl SettingsScreen {
    /// Draw the settings screen
    pub fn draw(f: &mut Frame, app: &App, selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Content
                Constraint::Length(3), // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0]);
        Self::draw_content(f, chunks[1], app, selected);
        Self::draw_status_bar(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect) {
        let header = Paragraph::new(Line::from(vec![Span::styled(
            " ⚙️ Settings ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(header, area);
    }

    fn draw_content(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(area);

        Self::draw_categories(f, chunks[0], selected);
        Self::draw_settings(f, chunks[1], app, selected);
    }

    fn draw_categories(f: &mut Frame, area: Rect, selected: usize) {
        let categories = SettingsCategory::all();

        let items: Vec<ListItem> = categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    Span::raw(cat.icon()),
                    Span::raw(" "),
                    Span::styled(cat.display_name(), style),
                ]))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title(" Categories ")
                .title_style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(list, area);
    }

    fn draw_settings(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let categories = SettingsCategory::all();
        let category = categories
            .get(selected)
            .unwrap_or(&SettingsCategory::General);

        let block = Block::default()
            .title(format!(" {} ", category.display_name()))
            .title_style(Style::default().fg(Color::Green))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        f.render_widget(block, area);

        match category {
            SettingsCategory::General => Self::draw_general_settings(f, inner, app),
            SettingsCategory::Removal => Self::draw_removal_settings(f, inner, app),
            SettingsCategory::Appearance => Self::draw_appearance_settings(f, inner, app),
        }
    }

    fn draw_general_settings(f: &mut Frame, area: Rect, app: &App) {
        let config = &app.state.config;

        let lines = vec![
            Line::from(""),
            Self::setting_line(
                "Dry Run Mode",
                if app.state.dry_run {
                    "Enabled"
                } else {
                    "Disabled"
                },
                if app.state.dry_run {
                    Color::Yellow
                } else {
                    Color::Green
                },
            ),
            Line::from(Span::styled(
                "  When enabled, no actual changes are made to the device",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Auto Update Check",
                if config.auto_update_check {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Check for new versions on startup",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Offline Mode",
                if config.offline_mode {
                    "Enabled"
                } else {
                    "Disabled"
                },
                if config.offline_mode {
                    Color::Yellow
                } else {
                    Color::Green
                },
            ),
            Line::from(Span::styled(
                "  Use local package database only",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Verbose Output",
                if config.verbose {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::White,
            ),
            Line::from(Span::styled(
                "  Show detailed operation logs",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    }

    fn draw_removal_settings(f: &mut Frame, area: Rect, app: &App) {
        let config = &app.state.config;

        let removal_mode = match config.removal_mode {
            bloatwarehatao_core::package::RemovalMode::Uninstall => "User-level Uninstall",
            bloatwarehatao_core::package::RemovalMode::Disable => "Disable Only",
            bloatwarehatao_core::package::RemovalMode::Clear => "Clear Data",
        };

        let lines = vec![
            Line::from(""),
            Self::setting_line("Default Mode", removal_mode, Color::Green),
            Line::from(Span::styled(
                "  Uninstall for current user only (recommended)",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Safety Warnings",
                if config.ui.show_safety_warnings {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Show warnings for risky packages",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Confirm Removal",
                if config.ui.confirm_removal {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Show confirmation before removal",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Backup Before Remove",
                if config.backup_before_remove {
                    "Always"
                } else {
                    "Never"
                },
                if config.backup_before_remove {
                    Color::Green
                } else {
                    Color::Red
                },
            ),
            Line::from(Span::styled(
                "  Create rescue list before removing packages",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Max Safety Rating",
                &config.max_safety_rating,
                Color::Yellow,
            ),
            Line::from(Span::styled(
                "  Maximum package safety level to show",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    }

    fn draw_appearance_settings(f: &mut Frame, area: Rect, app: &App) {
        let config = &app.state.config;

        let theme_text = match config.theme {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        };

        let lines = vec![
            Line::from(""),
            Self::setting_line("Theme", theme_text, Color::Cyan),
            Line::from(Span::styled(
                "  Application color theme",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Show Descriptions",
                if config.ui.show_descriptions {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Show package descriptions in lists",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Show Progress",
                if config.ui.show_progress {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Show progress indicators during operations",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Self::setting_line(
                "Animations",
                if config.ui.animations {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
            ),
            Line::from(Span::styled(
                "  Enable UI animations",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    }

    fn setting_line(name: &str, value: &str, value_color: Color) -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("  {} ", name), Style::default().fg(Color::White)),
            Span::styled(
                value.to_string(),
                Style::default()
                    .fg(value_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let help = Span::styled(
            " ↑/↓: Navigate | Enter: Change | ESC: Main Menu | q: Main Menu ",
            Style::default().fg(Color::DarkGray),
        );

        let dry_run = if app.state.dry_run {
            Span::styled(
                " 🧪 DRY RUN ",
                Style::default().fg(Color::Black).bg(Color::Yellow),
            )
        } else {
            Span::raw("")
        };

        let status_bar = Paragraph::new(Line::from(vec![dry_run, help])).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(status_bar, area);
    }
}
