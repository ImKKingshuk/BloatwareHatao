//! Support/Sponsor screen implementation
//!
//! Information for donations and support.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;
use crate::state::SupportType;

/// Support screen renderer
pub struct SupportScreen;

impl SupportScreen {
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Header (Fixed height)
                Constraint::Min(0),    // List Content (Takes remaining space)
                Constraint::Length(3), // Footer
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0]);
        Self::draw_content(f, chunks[1], app);
        Self::draw_footer(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(Span::styled(
                " 💖 Support Development ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("If you find this tool useful, please consider supporting development!"),
        ];

        // Center the content but use full width
        let p = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));

        f.render_widget(p, area);
    }

    fn draw_content(f: &mut Frame, area: Rect, app: &App) {
        // Create list items
        let items: Vec<ListItem> = app
            .state
            .support
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == app.state.support.selected;

                // Color logic
                let label_color = match item.type_ {
                    SupportType::GitHub => Color::Magenta,
                    SupportType::Crypto(_) => Color::Yellow,
                };

                let item_style = if is_selected {
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let label_span = Span::styled(
                    format!("{:<20}", item.label),
                    if is_selected {
                        Style::default()
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(label_color)
                            .add_modifier(Modifier::BOLD)
                    },
                );

                // Align value with padding
                let value_span = Span::styled(format!("  {}", item.value), item_style);

                let content = Line::from(vec![label_span, value_span]);

                ListItem::new(content)
            })
            .collect();

        // Calculate a centered area with max width but full height within parent 'area'
        // We want to avoid "huge blank space" on top, so we use most of the vertical space.
        // But we constrain width to keep it readable.
        let list_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(2),
                Constraint::Percentage(90), // Take 90% width
                Constraint::Min(2),
            ])
            .split(area)[1];

        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(" Select Option to Copy ")
            .border_style(Style::default().fg(Color::DarkGray));

        // Create a ListState just for rendering to handle scrolling
        let mut list_state = ListState::default();
        list_state.select(Some(app.state.support.selected));

        let list = List::new(items)
            .block(list_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray),
            ) // Backup highlight
            .highlight_symbol("▶ ");

        // Use render_stateful_widget to enable auto-scrolling
        f.render_stateful_widget(list, list_area, &mut list_state);
    }

    fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
        // Check for active status message
        let status_msg = if let Some((msg, time)) = &app.state.support.copy_status {
            if time.elapsed().as_secs() < 3 {
                Some(Span::styled(
                    format!(" ✅ {} ", msg),
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                None
            }
        } else {
            None
        };

        let help_text = vec![
            Span::raw("Press "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to Copy Address • "),
            Span::styled("ESC/q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to Back"),
        ];

        let content = if let Some(msg) = status_msg {
            Line::from(msg)
        } else {
            Line::from(help_text)
        };

        let footer = Paragraph::new(content)
            .block(Block::default().borders(Borders::TOP))
            .alignment(Alignment::Center);

        f.render_widget(footer, area);
    }
}
