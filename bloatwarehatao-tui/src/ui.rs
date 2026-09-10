//! UI rendering module

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

/// Main draw function
pub fn draw(f: &mut Frame, app: &App) {
    match app.current_screen {
        Screen::Home => draw_home(f, app),
        Screen::Help => draw_help(f, app),
        _ => draw_home(f, app), // Fallback for now
    }
}

/// Draw the home screen
fn draw_home(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Header/banner
            Constraint::Min(10),    // Menu
            Constraint::Length(3),  // Status bar
        ])
        .split(f.area());

    draw_banner(f, chunks[0]);
    draw_menu(f, chunks[1], app);
    draw_status_bar(f, chunks[2], app);
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

/// Draw the status bar
fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let dry_run_indicator = if app.dry_run {
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

/// Draw help screen
fn draw_help(f: &mut Frame, _app: &App) {
    let area = centered_rect(60, 80, f.area());

    let help_text = vec![
        Line::from(Span::styled(
            "BloatwareHatao - Keyboard Shortcuts",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ↑/k       Move up"),
        Line::from("  ↓/j       Move down"),
        Line::from("  Enter     Select/confirm"),
        Line::from("  Esc       Go back"),
        Line::from(""),
        Line::from(Span::styled("Actions", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  Space     Toggle selection"),
        Line::from("  /         Search packages"),
        Line::from("  f         Filter options"),
        Line::from("  r         Refresh list"),
        Line::from(""),
        Line::from(Span::styled("General", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ?         Show this help"),
        Line::from("  q         Quit application"),
        Line::from("  Ctrl+c    Force quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
