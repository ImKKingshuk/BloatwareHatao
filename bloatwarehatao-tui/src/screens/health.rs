//! Health screen implementation
//!
//! Displays device health metrics (battery, RAM, storage).

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::app::App;
use crate::state::DeviceState;
use bloatwarehatao_core::device::DeviceHealth;

/// Health screen renderer
pub struct HealthScreen;

impl HealthScreen {
    /// Draw the health screen
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Content
                Constraint::Length(3), // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0]);
        Self::draw_content(f, chunks[1], app);
        Self::draw_status_bar(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect) {
        let header = Paragraph::new(Line::from(vec![Span::styled(
            " ❤️ Device Health Check ",
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

    fn draw_content(f: &mut Frame, area: Rect, app: &App) {
        // Check if device is connected
        if !matches!(app.state.device, DeviceState::Connected(_)) {
            let block = Block::default()
                .title(" Health Metrics ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "❌ No device connected",
                    Style::default().fg(Color::Red),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Connect a device to view health metrics.",
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .block(block)
            .alignment(Alignment::Center);

            f.render_widget(msg, area);
            return;
        }

        let health_state = &app.state.health;

        if health_state.loading {
            let block = Block::default()
                .title(" Health Metrics ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "⏳ Loading health metrics...",
                    Style::default().fg(Color::Yellow),
                )),
            ])
            .block(block)
            .alignment(Alignment::Center);

            f.render_widget(msg, area);
            return;
        }

        if let Some(ref error) = health_state.error {
            let block = Block::default()
                .title(" Health Metrics ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("⚠️ Error: {}", error),
                    Style::default().fg(Color::Red),
                )),
            ])
            .block(block)
            .alignment(Alignment::Center);

            f.render_widget(msg, area);
            return;
        }

        if let Some(ref health) = health_state.health {
            Self::draw_health_metrics(f, area, health);
        }
    }

    fn draw_health_metrics(f: &mut Frame, area: Rect, health: &DeviceHealth) {
        let block = Block::default()
            .title(" Health Metrics ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(4), // Battery
                Constraint::Length(1), // Spacing
                Constraint::Length(4), // RAM
                Constraint::Length(1), // Spacing
                Constraint::Length(4), // Storage
                Constraint::Min(1),    // Remaining space
            ])
            .split(inner);

        // Battery section
        Self::draw_battery_section(f, chunks[0], health);

        // RAM section
        Self::draw_ram_section(f, chunks[2], health);

        // Storage section
        Self::draw_storage_section(f, chunks[4], health);
    }

    fn draw_battery_section(f: &mut Frame, area: Rect, health: &DeviceHealth) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(2)])
            .split(area);

        // Battery label
        let battery_pct = health.battery_level.unwrap_or(0);
        let temp = health
            .battery_temp_celsius()
            .map(|t| format!(" ({:.1}°C)", t))
            .unwrap_or_default();

        let battery_icon = if battery_pct > 20 { "🔋" } else { "🪫" };

        let label = Line::from(vec![Span::styled(
            format!("{} Battery: {}%{}", battery_icon, battery_pct, temp),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]);
        f.render_widget(Paragraph::new(label), chunks[0]);

        // Battery gauge
        let battery_color = if battery_pct > 50 {
            Color::Green
        } else if battery_pct > 20 {
            Color::Yellow
        } else {
            Color::Red
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(battery_color).bg(Color::DarkGray))
            .percent(battery_pct as u16)
            .label(format!("{}%", battery_pct));

        f.render_widget(gauge, chunks[1]);
    }

    fn draw_ram_section(f: &mut Frame, area: Rect, health: &DeviceHealth) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(2)])
            .split(area);

        // RAM label
        let ram_pct = health.ram_usage_percent().unwrap_or(0.0) as u32;
        let ram_total_mb = health.ram_total_kb.unwrap_or(0) / 1024;
        let ram_available_mb = health.ram_available_kb.unwrap_or(0) / 1024;
        let ram_used_mb = ram_total_mb.saturating_sub(ram_available_mb);

        let label = Line::from(vec![Span::styled(
            format!(
                "💾 RAM: {} MB / {} MB ({}%)",
                ram_used_mb, ram_total_mb, ram_pct
            ),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]);
        f.render_widget(Paragraph::new(label), chunks[0]);

        // RAM gauge
        let ram_color = if ram_pct < 70 {
            Color::Green
        } else if ram_pct < 90 {
            Color::Yellow
        } else {
            Color::Red
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(ram_color).bg(Color::DarkGray))
            .percent(ram_pct.min(100) as u16)
            .label(format!("{}%", ram_pct));

        f.render_widget(gauge, chunks[1]);
    }

    fn draw_storage_section(f: &mut Frame, area: Rect, health: &DeviceHealth) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(2)])
            .split(area);

        // Storage label
        let storage_pct = health.storage_usage_percent().unwrap_or(0.0) as u32;
        let storage_total_gb = health.storage_total_kb.unwrap_or(0) / 1024 / 1024;
        let storage_free_gb = health.storage_free_kb.unwrap_or(0) / 1024 / 1024;
        let storage_used_gb = storage_total_gb.saturating_sub(storage_free_gb);

        let label = Line::from(vec![Span::styled(
            format!(
                "📂 Storage: {} GB / {} GB ({}%)",
                storage_used_gb, storage_total_gb, storage_pct
            ),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]);
        f.render_widget(Paragraph::new(label), chunks[0]);

        // Storage gauge
        let storage_color = if storage_pct < 70 {
            Color::Green
        } else if storage_pct < 90 {
            Color::Yellow
        } else {
            Color::Red
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(storage_color).bg(Color::DarkGray))
            .percent(storage_pct.min(100) as u16)
            .label(format!("{}%", storage_pct));

        f.render_widget(gauge, chunks[1]);
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let help = Span::styled(
            " r: Refresh | ESC: Main Menu | q: Main Menu ",
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
