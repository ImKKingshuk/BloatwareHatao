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
/// Settings categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    General,
    Removal,
}

impl SettingsCategory {
    pub fn all() -> Vec<Self> {
        vec![Self::General, Self::Removal]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Removal => "Removal Options",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::General => "⚙️",
            Self::Removal => "🗑️",
        }
    }
}

/// Settings screen renderer
pub struct SettingsScreen;

impl SettingsScreen {
    pub const fn category_count() -> usize {
        2
    }

    pub const fn item_count(category: usize) -> usize {
        match category {
            0 => 1,
            1 => 2,
            _ => 0,
        }
    }

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
        Self::draw_content(
            f,
            chunks[1],
            app,
            selected,
            app.state.settings_item_selected,
        );
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

    fn draw_content(f: &mut Frame, area: Rect, app: &App, selected: usize, item_selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(area);

        Self::draw_categories(f, chunks[0], selected);
        Self::draw_settings(f, chunks[1], app, selected, item_selected);
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

    fn draw_settings(f: &mut Frame, area: Rect, app: &App, selected: usize, item_selected: usize) {
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
            SettingsCategory::General => Self::draw_general_settings(f, inner, app, item_selected),
            SettingsCategory::Removal => Self::draw_removal_settings(f, inner, app, item_selected),
        }
    }

    fn draw_general_settings(f: &mut Frame, area: Rect, app: &App, selected: usize) {
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
                selected == 0,
            ),
            Line::from(Span::styled(
                "  When enabled, no actual changes are made to the device",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    }

    fn draw_removal_settings(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let config = &app.state.config;

        let lines = vec![
            Line::from(""),
            Self::setting_line(
                "Confirm Removal",
                if config.ui.confirm_removal {
                    "Enabled"
                } else {
                    "Disabled"
                },
                Color::Green,
                selected == 0,
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
                selected == 1,
            ),
            Line::from(Span::styled(
                "  Create rescue list before removing packages",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    }

    fn setting_line(name: &str, value: &str, value_color: Color, selected: bool) -> Line<'static> {
        let prefix = if selected { "▶ " } else { "  " };
        let name_style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        Line::from(vec![
            Span::raw(prefix),
            Span::styled(format!("{} ", name), name_style),
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
            " ←/→: Category | ↑/↓: Setting | Enter: Change | s: Save | ESC/q: Main Menu ",
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
