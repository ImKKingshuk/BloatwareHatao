//! Rescue History screen implementation
//!
//! Manage rescue history and restore packages.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};


use crate::app::App;

/// Rescue History screen renderer
pub struct RescueScreen;

impl RescueScreen {
    /// Draw the rescue screen using rescue history from state
    pub fn draw(f: &mut Frame, app: &App, tab: usize, selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header with tabs
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0], tab);
        Self::draw_content(f, chunks[1], app, tab, selected);
        Self::draw_status_bar(f, chunks[2], app, tab);
    }

    fn draw_header(f: &mut Frame, area: Rect, tab: usize) {
        let tab1_style = if tab == 0 {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let tab2_style = if tab == 1 {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let header = Paragraph::new(Line::from(vec![
            Span::styled(" 🚑 Rescue History ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(" Rescue Points ", tab1_style),
            Span::raw("  "),
            Span::styled(" Sessions ", tab2_style),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(header, area);
    }

    fn draw_content(f: &mut Frame, area: Rect, app: &App, tab: usize, selected: usize) {
        match tab {
            0 => Self::draw_rescue_points_tab(f, area, app, selected),
            1 => Self::draw_sessions_tab(f, area, app, selected),
            _ => {},
        }
    }

    fn draw_rescue_points_tab(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Self::draw_rescue_list(f, chunks[0], app, selected);
        Self::draw_rescue_details(f, chunks[1], app, selected);
    }

    fn draw_rescue_list(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let entries = &app.state.rescue_entries;
        
        if entries.is_empty() {
            let empty = Paragraph::new(vec![
                Line::from(""),
                Line::from("No rescue points found"),
                Line::from(""),
                Line::from(Span::styled(
                    "Press 'n' to create a new rescue point",
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Rescue Points ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
            f.render_widget(empty, area);
            return;
        }

        let items: Vec<ListItem> = entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let date_str = entry.created_at.format("%Y-%m-%d %H:%M").to_string();
                
                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(prefix),
                        Span::styled("📁 ", Style::default().fg(Color::Yellow)),
                        Span::styled(&entry.id, style),
                    ]),
                    Line::from(Span::styled(
                        format!("    {} • {} packages", date_str, entry.packages.len()),
                        Style::default().fg(Color::DarkGray),
                    )),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Rescue Points ({}) ", entries.len()))
                    .title_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(list, area);
    }

    fn draw_rescue_details(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let entries = &app.state.rescue_entries;
        
        let block = Block::default()
            .title(" Details ")
            .title_style(Style::default().fg(Color::Green))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let Some(entry) = entries.get(selected) else {
            let empty = Paragraph::new("Select a rescue point to view details")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(empty, area);
            return;
        };

        let date_str = entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let device_info = entry.device_model
            .as_deref()
            .unwrap_or("Unknown device");
        
        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &entry.id,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&date_str, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Device: ", Style::default().fg(Color::DarkGray)),
                Span::styled(device_info, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Packages: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", entry.packages.len()),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(""),
            if let Some(desc) = &entry.description {
                Line::from(vec![
                    Span::styled("Note: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(desc, Style::default().fg(Color::White)),
                ])
            } else {
                Line::from("")
            },
            Line::from(""),
            Line::from(Span::styled(
                "Actions:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  Enter - Restore from rescue point",
                Style::default().fg(Color::Green),
            )),
            Line::from(Span::styled(
                "  v - View package list",
                Style::default().fg(Color::White),
            )),
            Line::from(Span::styled(
                "  d - Delete rescue point",
                Style::default().fg(Color::Red),
            )),
        ];

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn draw_sessions_tab(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Self::draw_session_list(f, chunks[0], app, selected);
        Self::draw_session_details(f, chunks[1], app, selected);
    }

    fn draw_session_list(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let sessions = &app.state.rescue_sessions;
        
        if sessions.is_empty() {
            let empty = Paragraph::new(vec![
                Line::from(""),
                Line::from("No rescue sessions found"),
                Line::from(""),
                Line::from(Span::styled(
                    "Sessions are created automatically",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "when you remove packages",
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Sessions ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
            f.render_widget(empty, area);
            return;
        }

        let items: Vec<ListItem> = sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let date_str = session.created_at.format("%Y-%m-%d %H:%M").to_string();
                
                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(prefix),
                        Span::styled("🚑 ", Style::default().fg(Color::Red)),
                        Span::styled(&session.session_id, style),
                    ]),
                    Line::from(Span::styled(
                        format!("    {} • {} removed", date_str, session.removed_packages.len()),
                        Style::default().fg(Color::DarkGray),
                    )),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Sessions ({}) ", sessions.len()))
                    .title_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(list, area);
    }

    fn draw_session_details(f: &mut Frame, area: Rect, app: &App, selected: usize) {
        let sessions = &app.state.rescue_sessions;
        
        let block = Block::default()
            .title(" Details ")
            .title_style(Style::default().fg(Color::Green))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let Some(session) = sessions.get(selected) else {
            let empty = Paragraph::new("Select a session to view details")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(empty, area);
            return;
        };

        let date_str = session.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &session.session_id,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&date_str, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Packages: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", session.removed_packages.len()),
                    Style::default().fg(Color::Red),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Removed Packages:",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        // Add first 10 packages
        for pkg in session.removed_packages.iter().take(10) {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(&pkg.name, Style::default().fg(Color::White)),
            ]));
        }

        if session.removed_packages.len() > 10 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", session.removed_packages.len() - 10),
                Style::default().fg(Color::DarkGray),
            )));
        }

        lines.extend(vec![
            Line::from(""),
            Line::from(Span::styled(
                "Actions:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  Enter - Restore packages",
                Style::default().fg(Color::Green),
            )),
        ]);

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App, tab: usize) {
        let help = if tab == 0 {
            " Tab: Switch | ↑/↓: Navigate | Enter: Restore | n: New | r: Reload | d: Delete | ESC: Main Menu "
        } else {
            " Tab: Switch | ↑/↓: Navigate | Enter: Restore | ESC: Main Menu "
        };

        let help_span = Span::styled(help, Style::default().fg(Color::DarkGray));

        let dry_run = if app.state.dry_run {
            Span::styled(" 🧪 DRY RUN ", Style::default().fg(Color::Black).bg(Color::Yellow))
        } else {
            Span::raw("")
        };

        let status_bar = Paragraph::new(Line::from(vec![dry_run, help_span]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(status_bar, area);
    }
}
