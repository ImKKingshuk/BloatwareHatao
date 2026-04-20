//! Dialog components for TUI
//!
//! Reusable dialog widgets for confirmations, progress, and results.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
};

use crate::state::{DialogState, DialogType, PackageOperation};

/// Draw dialogs overlay
pub fn draw_dialogs(f: &mut Frame, dialog: &DialogState) {
    let Some(ref dialog_type) = dialog.active else {
        return;
    };

    match dialog_type {
        DialogType::Confirm {
            title,
            message,
            operation,
            packages,
        } => {
            draw_confirm_dialog(f, title, message, *operation, packages, dialog.selected);
        }
        DialogType::Progress {
            title,
            current,
            total,
            current_package,
        } => {
            draw_progress_dialog(f, title, *current, *total, current_package);
        }
        DialogType::Result {
            title,
            success,
            failed,
        } => {
            draw_result_dialog(f, title, success, failed);
        }
        DialogType::ActionMenu { selected } => {
            draw_action_menu(f, *selected);
        }
        DialogType::Error { title, message } => {
            draw_error_dialog(f, title, message);
        }
    }
}

/// Draw confirmation dialog
fn draw_confirm_dialog(
    f: &mut Frame,
    title: &str,
    message: &str,
    operation: PackageOperation,
    packages: &[String],
    selected: usize,
) {
    let area = centered_rect(50, 40, f.area());
    f.render_widget(Clear, area);

    let border_color = if operation.is_destructive() {
        Color::Red
    } else {
        Color::Yellow
    };

    let block = Block::default()
        .title(format!(" {} {} ", operation.icon(), title))
        .title_style(
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Message
            Constraint::Min(3),    // Package list
            Constraint::Length(3), // Buttons
        ])
        .split(inner);

    // Message
    let msg = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(msg, chunks[0]);

    // Package list (scrollable preview)
    let package_items: Vec<ListItem> = packages
        .iter()
        .take(5)
        .map(|p| {
            ListItem::new(Span::styled(
                format!("  • {}", p),
                Style::default().fg(Color::Cyan),
            ))
        })
        .collect();

    let mut items = package_items;
    if packages.len() > 5 {
        items.push(ListItem::new(Span::styled(
            format!("  ... and {} more", packages.len() - 5),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    // Buttons
    let yes_style = if selected == 0 {
        Style::default().fg(Color::Black).bg(Color::Green)
    } else {
        Style::default().fg(Color::Green)
    };

    let no_style = if selected == 1 {
        Style::default().fg(Color::Black).bg(Color::Red)
    } else {
        Style::default().fg(Color::Red)
    };

    let buttons = Paragraph::new(Line::from(vec![
        Span::raw("    "),
        Span::styled(" Yes ", yes_style.add_modifier(Modifier::BOLD)),
        Span::raw("     "),
        Span::styled(" No ", no_style.add_modifier(Modifier::BOLD)),
    ]))
    .alignment(Alignment::Center);

    f.render_widget(buttons, chunks[2]);
}

/// Draw progress dialog
fn draw_progress_dialog(
    f: &mut Frame,
    title: &str,
    current: usize,
    total: usize,
    current_package: &str,
) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(format!(" ⏳ {} ", title))
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Status text
            Constraint::Length(2), // Progress bar
            Constraint::Length(1), // Current package
        ])
        .split(inner);

    // Status
    let status = Paragraph::new(format!("Processing {} of {}", current + 1, total))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(status, chunks[0]);

    // Progress bar
    let percent = if total > 0 {
        ((current as f64 / total as f64) * 100.0) as u16
    } else {
        100
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(percent)
        .label(format!("{}%", percent));
    f.render_widget(gauge, chunks[1]);

    // Current package
    let pkg = Paragraph::new(current_package)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(pkg, chunks[2]);
}

/// Draw result dialog
fn draw_result_dialog(f: &mut Frame, title: &str, success: &[String], failed: &[(String, String)]) {
    let area = centered_rect(60, 50, f.area());
    f.render_widget(Clear, area);

    let (icon, color) = if failed.is_empty() {
        ("✅", Color::Green)
    } else if success.is_empty() {
        ("❌", Color::Red)
    } else {
        ("⚠️", Color::Yellow)
    };

    let block = Block::default()
        .title(format!(" {} {} ", icon, title))
        .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Summary
            Constraint::Min(5),    // Details
            Constraint::Length(2), // Close hint
        ])
        .split(inner);

    // Summary
    let summary = Paragraph::new(format!(
        "{} succeeded, {} failed",
        success.len(),
        failed.len()
    ))
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center);
    f.render_widget(summary, chunks[0]);

    // Details
    let mut items = Vec::new();

    if !success.is_empty() {
        items.push(ListItem::new(Span::styled(
            "✓ Succeeded:",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        for pkg in success.iter().take(5) {
            items.push(ListItem::new(Span::styled(
                format!("  {}", pkg),
                Style::default().fg(Color::Green),
            )));
        }
        if success.len() > 5 {
            items.push(ListItem::new(Span::styled(
                format!("  ... and {} more", success.len() - 5),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    if !failed.is_empty() {
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Span::styled(
            "✗ Failed:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        for (pkg, err) in failed.iter().take(5) {
            items.push(ListItem::new(Span::styled(
                format!("  {}: {}", pkg, err),
                Style::default().fg(Color::Red),
            )));
        }
        if failed.len() > 5 {
            items.push(ListItem::new(Span::styled(
                format!("  ... and {} more", failed.len() - 5),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let list = List::new(items);
    f.render_widget(list, chunks[1]);

    // Close hint
    let hint = Paragraph::new("Press Enter or Esc to close")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}

/// Draw action menu
fn draw_action_menu(f: &mut Frame, selected: usize) {
    let area = centered_rect(35, 35, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" 📋 Actions ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let actions = [
        (PackageOperation::Uninstall, "Remove package from device"),
        (PackageOperation::Disable, "Disable without removing"),
        (PackageOperation::Enable, "Re-enable disabled package"),
        (PackageOperation::ClearData, "Clear app data and cache"),
    ];

    let items: Vec<ListItem> = actions
        .iter()
        .enumerate()
        .map(|(i, (op, desc))| {
            let style = if i == selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == selected { "▶ " } else { "  " };
            let icon_color = if op.is_destructive() {
                Color::Red
            } else {
                Color::Green
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(op.icon(), Style::default().fg(icon_color)),
                    Span::raw(" "),
                    Span::styled(op.display_name(), style),
                ]),
                Line::from(Span::styled(
                    format!("    {}", desc),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
        })
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    let list = List::new(items);
    f.render_widget(list, chunks[0]);

    let hint = Paragraph::new("↑/↓: Select | Enter: Confirm | Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

/// Draw error dialog
fn draw_error_dialog(f: &mut Frame, title: &str, message: &str) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(format!(" ❌ {} ", title))
        .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    let msg = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(msg, chunks[0]);

    let hint = Paragraph::new("Press Enter or Esc to close")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

/// Create a centered rectangle
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

/// Get available operations (for action menu navigation)
pub fn action_count() -> usize {
    4 // Uninstall, Disable, Enable, ClearData
}

/// Get operation by index
pub fn operation_by_index(index: usize) -> Option<PackageOperation> {
    match index {
        0 => Some(PackageOperation::Uninstall),
        1 => Some(PackageOperation::Disable),
        2 => Some(PackageOperation::Enable),
        3 => Some(PackageOperation::ClearData),
        _ => None,
    }
}
