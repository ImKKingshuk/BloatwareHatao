//! Removal Presets screen implementation
//!
//! View and apply removal presets.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use bloatwarehatao_core::preset::PresetType;
use crate::app::App;
use std::cell::RefCell;
use std::collections::HashSet;
use ratatui::widgets::ListState;

/// Preset creation step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetCreationStep {
    NameInput,
    DescriptionInput,
    PackageSelection,
    #[allow(dead_code)]
    Confirm,
}

/// Preset creator state
#[derive(Debug, Clone)]
pub struct PresetCreatorState {
    pub active: bool,
    pub step: PresetCreationStep,
    pub name_input: String,
    pub description_input: String,
    pub available_packages: Vec<String>,
    pub selected_packages: HashSet<String>,
    pub list_state: RefCell<ListState>,
}

impl Default for PresetCreatorState {
    fn default() -> Self {
        Self {
            active: false,
            step: PresetCreationStep::NameInput,
            name_input: String::new(),
            description_input: String::new(),
            available_packages: Vec::new(),
            selected_packages: HashSet::new(),
            list_state: RefCell::new(ListState::default()),
        }
    }
}

/// Removal Presets screen renderer
pub struct PresetsScreen;

impl PresetsScreen {
    /// Draw the presets screen using presets from state
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.area());

        Self::draw_header(f, chunks[0], app);
        Self::draw_content(f, chunks[1], app);
        Self::draw_status_bar(f, chunks[2], app);
    }

    fn draw_header(f: &mut Frame, area: Rect, app: &App) {
        let title = if app.state.preset_creator.active {
            " ✨ Create New Preset "
        } else {
            " 📋 Removal Presets "
        };
        
        let color = if app.state.preset_creator.active {
            Color::Yellow
        } else {
            Color::Cyan
        };

        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                title,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
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
        if app.state.preset_creator.active {
            Self::draw_creator(f, area, app);
        } else {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            Self::draw_preset_list(f, chunks[0], app);
            Self::draw_preset_details(f, chunks[1], app);
        }
    }

    fn draw_creator(f: &mut Frame, area: Rect, app: &App) {
        let state = &app.state.preset_creator;
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
            
        let inner = block.inner(area);
        f.render_widget(block, area);

        match state.step {
            PresetCreationStep::NameInput => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
                    .split(inner);

                f.render_widget(Paragraph::new("Step 1/3: Preset Name").alignment(Alignment::Center), chunks[0]);

                let input = Paragraph::new(state.name_input.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL).title(" Name "));
                f.render_widget(input, chunks[1]);
                
                f.render_widget(Paragraph::new("Enter a unique name for this preset.\nPress Enter to continue.").style(Style::default().fg(Color::DarkGray)), chunks[2]);
            }
            PresetCreationStep::DescriptionInput => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Length(6), Constraint::Min(0)])
                    .split(inner);

                f.render_widget(Paragraph::new("Step 2/3: Description").alignment(Alignment::Center), chunks[0]);

                let input = Paragraph::new(state.description_input.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL).title(" Description "));
                f.render_widget(input, chunks[1]);
                
                 f.render_widget(Paragraph::new("Enter a brief description.\nPress Enter to continue.").style(Style::default().fg(Color::DarkGray)), chunks[2]);
            }
            PresetCreationStep::PackageSelection => {
                 let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(2), Constraint::Min(0), Constraint::Length(1)])
                    .split(inner);
                    
                 let instructions = Paragraph::new("Step 3/3: Select Packages (Space to toggle, Enter to save)")
                    .style(Style::default().fg(Color::Green));
                 f.render_widget(instructions, chunks[0]);
                 
                 let items: Vec<ListItem> = state.available_packages.iter().map(|s| {
                     let selected = state.selected_packages.contains(s);
                     let checkbox = if selected { "[x] " } else { "[ ] " };
                     let style = if selected { Style::default().fg(Color::Green) } else { Style::default() };
                     ListItem::new(format!("{}{}", checkbox, s)).style(style)
                 }).collect();
                 
                 let list = List::new(items)
                    .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));
                 
                 let mut list_state = state.list_state.borrow_mut();
                 f.render_stateful_widget(list, chunks[1], &mut *list_state);

                 let count = state.selected_packages.len();
                 let footer = Paragraph::new(format!("Selected: {} packages", count)).alignment(Alignment::Right);
                 f.render_widget(footer, chunks[2]);
            }
            PresetCreationStep::Confirm => {
                 // Should not stay here long, handled by key press logic usually or just skip to save
            }
        }
    }

    fn draw_preset_list(f: &mut Frame, area: Rect, app: &App) {
        let presets = &app.state.presets;
        let selected = app.state.presets_selected;
        
        if presets.is_empty() {
            let empty = Paragraph::new("No presets available")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title(" Presets ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );
            f.render_widget(empty, area);
            return;
        }

        let items: Vec<ListItem> = presets
            .iter()
            .enumerate()
            .map(|(i, preset)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == selected { "▶ " } else { "  " };
                let icon = match preset.preset_type {
                    PresetType::BuiltIn => "📦",
                    PresetType::Custom => "📝",
                    PresetType::Community => "🌐",
                };
                
                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(prefix),
                        Span::raw(icon),
                        Span::raw(" "),
                        Span::styled(&preset.name, style),
                    ]),
                    Line::from(Span::styled(
                        format!("    {} packages", preset.packages.len()),
                        Style::default().fg(Color::DarkGray),
                    )),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Presets ({}) ", presets.len()))
                    .title_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(list, area);
    }

    fn draw_preset_details(f: &mut Frame, area: Rect, app: &App) {
        let presets = &app.state.presets;
        let selected = app.state.presets_selected;
        
        let block = Block::default()
            .title(" Preset Details ")
            .title_style(Style::default().fg(Color::Green))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let Some(preset) = presets.get(selected) else {
            let empty = Paragraph::new("Select a preset to view details")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(empty, area);
            return;
        };

        let (type_color, type_text) = match preset.preset_type {
            PresetType::BuiltIn => (Color::Cyan, "Built-in"),
            PresetType::Custom => (Color::Yellow, "Custom"),
            PresetType::Community => (Color::Magenta, "Community"),
        };

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &preset.name,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
                Span::styled(type_text, Style::default().fg(type_color)),
            ]),
            Line::from(vec![
                Span::styled("Packages: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", preset.packages.len()),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&preset.id, Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Description:", Style::default().fg(Color::DarkGray))),
            Line::from(Span::styled(
                &preset.description,
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to apply this preset",
                Style::default().fg(Color::Green),
            )),
            Line::from(Span::styled(
                "Press 'p' to preview packages",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let help_text = if app.state.preset_creator.active {
            " Enter: Confirm | Tab: Next Step | ESC: Cancel | Space: Toggle Selection "
        } else {
            " ↑/↓: Navigate | Enter: Apply | n: New | e: Export | i: Import | d: Delete | q: Quit "
        };
        
        let help = Span::styled(
            help_text,
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
