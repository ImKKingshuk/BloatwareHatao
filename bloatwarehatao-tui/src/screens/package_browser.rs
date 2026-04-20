//! Package browser screen implementation
//!
//! Interactive package browser with search, filter, and selection.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
};

use crate::app::App;
use crate::state::{PackageItem, SelectionMode, StatusTab};
use bloatwarehatao_core::category::PackageCategory;
use bloatwarehatao_core::database::SafetyRating;

/// Package browser screen renderer
pub struct PackageBrowserScreen;

impl PackageBrowserScreen {
    /// Draw the package browser screen
    pub fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tabs bar
                Constraint::Length(3), // Header with search
                Constraint::Length(7), // Filter panel (multi-row)
                Constraint::Min(6),    // Package list + details
                Constraint::Length(3), // Status bar
            ])
            .split(f.area());

        Self::draw_tabs(f, chunks[0], app);
        Self::draw_header(f, chunks[1], app);
        Self::draw_filter_panel(f, chunks[2], app);
        Self::draw_main_area(f, chunks[3], app);
        Self::draw_status_bar(f, chunks[4], app);
    }

    /// Draw the tabs bar (All Apps | Installed | Disabled | Uninstalled)
    fn draw_tabs(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;
        let current_tab = browser.filter.status_tab;

        // Count packages for each tab
        let all_count = browser.packages.len();
        let installed_count = browser.packages.iter().filter(|p| p.installed).count();
        let disabled_count = 0_usize; // TODO: implement disabled tracking
        let uninstalled_count = browser.packages.iter().filter(|p| !p.installed).count();

        let tabs = [
            (StatusTab::All, all_count),
            (StatusTab::Installed, installed_count),
            (StatusTab::Disabled, disabled_count),
            (StatusTab::Uninstalled, uninstalled_count),
        ];

        let mut spans: Vec<Span> = vec![Span::raw(" ")];

        for (tab, count) in tabs.iter() {
            let is_selected = *tab == current_tab;
            let label = format!(" {} ({}) ", tab.label(), count);

            if is_selected {
                spans.push(Span::styled(
                    label,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(label, Style::default().fg(Color::DarkGray)));
            }
            spans.push(Span::raw(" │ "));
        }

        // Remove last separator
        spans.pop();
        spans.push(Span::styled(
            "  ◄► Tab/Shift+Tab to switch",
            Style::default().fg(Color::DarkGray),
        ));

        let tabs_bar = Paragraph::new(Line::from(spans)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(tabs_bar, area);
    }

    /// Draw the filter panel with safety levels and categories (multi-row)
    fn draw_filter_panel(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;

        // Get unique categories first to calculate row count
        let mut unique_cats: Vec<PackageCategory> = browser
            .packages
            .iter()
            .map(|p| p.category)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        unique_cats.sort_by(|a, b| a.display_name().cmp(b.display_name()));

        // Calculate how many category rows we need (8 per row)
        let cats_per_row = 8;
        let cat_row_count = unique_cats.len().div_ceil(cats_per_row);
        let cat_row_count = cat_row_count.clamp(1, 3); // Max 3 rows of categories

        // Split area: 1 row safety + N rows categories
        let mut constraints = vec![Constraint::Length(1)]; // Safety row
        for _ in 0..cat_row_count {
            constraints.push(Constraint::Length(1)); // Category rows
        }
        constraints.push(Constraint::Min(0)); // Spacing

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .margin(1)
            .split(area);

        // Row 1: Safety Level
        let safety_levels = [
            (SafetyRating::Recommended, "1:Recommended", Color::Green),
            (SafetyRating::Advanced, "2:Advanced", Color::Yellow),
            (SafetyRating::Unsafe, "3:Unsafe", Color::Rgb(255, 165, 0)),
            (SafetyRating::Danger, "4:Danger", Color::Red),
        ];

        let mut safety_spans: Vec<Span> = vec![Span::styled(
            "SAFETY: ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )];

        for (rating, label, color) in safety_levels.iter() {
            let is_active = browser.filter.safety == Some(*rating);
            let style = if is_active {
                Style::default()
                    .bg(*color)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(*color)
            };
            safety_spans.push(Span::styled(format!(" {} ", label), style));
            safety_spans.push(Span::raw(" "));
        }
        safety_spans.push(Span::styled(" ", Style::default().fg(Color::Red)));

        f.render_widget(Paragraph::new(Line::from(safety_spans)), rows[0]);

        // Category rows - split into multiple rows
        for row_idx in 0..cat_row_count {
            let start_idx = row_idx * cats_per_row;
            let end_idx = (start_idx + cats_per_row).min(unique_cats.len());

            let mut cat_spans: Vec<Span> = vec![];

            if row_idx == 0 {
                cat_spans.push(Span::styled(
                    "CATEGORY:",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                cat_spans.push(Span::styled("         ", Style::default())); // Align with first row
            }

            for (i, cat) in unique_cats[start_idx..end_idx].iter().enumerate() {
                let global_idx = start_idx + i;
                let key = if global_idx < 26 {
                    (b'A' + global_idx as u8) as char
                } else {
                    '?'
                };
                let is_active = browser.filter.category == Some(*cat);
                let style = if is_active {
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                cat_spans.push(Span::styled(
                    format!(" {}:{} ", key, cat.display_name()),
                    style,
                ));
            }

            f.render_widget(Paragraph::new(Line::from(cat_spans)), rows[1 + row_idx]);
        }

        // Draw border around entire panel
        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title_style(
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                )
                .title(
                    " Filters: (Safety:1-4, Category:A-Z, 0:Clear All, Press keys on Keyboard) ",
                ),
            area,
        );
    }

    /// Draw the header with title and inline search bar
    fn draw_header(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;

        // Split header into title area and search bar area
        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Title
                Constraint::Percentage(50), // Search bar
            ])
            .split(area);

        // Left side: Title and counts
        let title = format!(
            " 📦 Package Browser - {} packages ",
            browser.filtered_indices.len()
        );

        let selected_info = if browser.selected_count() > 0 {
            format!(" ☑ {} selected ", browser.selected_count())
        } else {
            String::new()
        };

        let title_widget = Paragraph::new(Line::from(vec![
            Span::styled(
                &title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&selected_info, Style::default().fg(Color::Green)),
        ]))
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(title_widget, header_chunks[0]);

        // Right side: Search bar
        let search_content = if browser.search_active {
            // Active search: show input with cursor
            Line::from(vec![
                Span::styled(" 🔍 ", Style::default().fg(Color::Yellow)),
                Span::styled(&browser.filter.search, Style::default().fg(Color::White)),
                Span::styled(
                    "█",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ])
        } else if !browser.filter.search.is_empty() {
            // Has search term but not active
            Line::from(vec![
                Span::styled(" 🔍 ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("\"{}\"", &browser.filter.search),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    "  (/ to edit, c to clear)",
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        } else {
            // No search, show placeholder
            Line::from(vec![Span::styled(
                " 🔍 Press / to search...",
                Style::default().fg(Color::DarkGray),
            )])
        };

        let search_bar = Paragraph::new(search_content).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if browser.search_active {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::DarkGray)
                }),
        );

        f.render_widget(search_bar, header_chunks[1]);
    }

    /// Draw the main content area (list + details)
    fn draw_main_area(f: &mut Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        Self::draw_package_list(f, chunks[0], app);
        Self::draw_package_details(f, chunks[1], app);
    }

    /// Draw the package list
    fn draw_package_list(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;

        if browser.loading {
            let loading = Paragraph::new("⏳ Loading packages...")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title(" Packages ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );
            f.render_widget(loading, area);
            return;
        }

        // Calculate visible area (accounting for borders)
        let inner_height = area.height.saturating_sub(2) as usize;

        // Calculate scroll offset to keep cursor visible
        let visible_start = if browser.cursor >= inner_height {
            browser.cursor - inner_height + 1
        } else {
            0
        };

        // Build list items
        let items: Vec<ListItem> = browser
            .filtered_indices
            .iter()
            .enumerate()
            .skip(visible_start)
            .take(inner_height)
            .filter_map(|(display_idx, &pkg_idx)| {
                let pkg = browser.packages.get(pkg_idx)?;
                let is_selected = display_idx == browser.cursor;
                Some(Self::make_list_item(
                    pkg,
                    is_selected,
                    browser.selection_mode,
                ))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title(" Packages ")
                .title_style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(list, area);

        // Render scrollbar
        if browser.filtered_indices.len() > inner_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state =
                ScrollbarState::new(browser.filtered_indices.len()).position(browser.cursor);

            f.render_stateful_widget(
                scrollbar,
                area.inner(ratatui::layout::Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            );
        }
    }

    /// Create a list item for a package
    fn make_list_item<'a>(
        pkg: &'a PackageItem,
        is_cursor: bool,
        mode: SelectionMode,
    ) -> ListItem<'a> {
        let safety_indicator = match pkg.safety {
            SafetyRating::Recommended => Span::styled("●", Style::default().fg(Color::Green)),
            SafetyRating::Advanced => Span::styled("●", Style::default().fg(Color::Yellow)),
            SafetyRating::Unsafe => Span::styled("●", Style::default().fg(Color::Rgb(255, 165, 0))), // Orange
            SafetyRating::Danger => Span::styled("●", Style::default().fg(Color::Red)),
            SafetyRating::User => Span::styled("●", Style::default().fg(Color::Blue)),
        };

        // Selection checkbox (in multi mode)
        let checkbox = match mode {
            SelectionMode::Multi => {
                if pkg.selected {
                    Span::styled("[✓] ", Style::default().fg(Color::Green))
                } else {
                    Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
                }
            }
            SelectionMode::Single => Span::raw(""),
        };

        // Cursor indicator
        let cursor = if is_cursor {
            Span::styled("▶ ", Style::default().fg(Color::Cyan))
        } else {
            Span::raw("  ")
        };

        // Package label
        let label_style = if is_cursor {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if pkg.selected {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        // Installed indicator
        let installed = if pkg.installed {
            Span::styled(" ✓", Style::default().fg(Color::Green))
        } else {
            Span::styled(" ○", Style::default().fg(Color::DarkGray))
        };

        ListItem::new(Line::from(vec![
            cursor,
            checkbox,
            safety_indicator,
            Span::raw(" "),
            Span::styled(&pkg.label, label_style),
            installed,
        ]))
    }

    /// Draw package details panel
    fn draw_package_details(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;

        let block = Block::default()
            .title(" Package Details ")
            .title_style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let Some(pkg) = browser.selected_package() else {
            let empty = Paragraph::new("No package selected")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(empty, area);
            return;
        };

        // Build details content
        let safety_color = match pkg.safety {
            SafetyRating::Recommended => Color::Green,
            SafetyRating::Advanced => Color::Yellow,
            SafetyRating::Unsafe => Color::Rgb(255, 165, 0),
            SafetyRating::Danger => Color::Red,
            SafetyRating::User => Color::Blue,
        };

        let mut lines = vec![
            Line::from(Span::styled(
                &pkg.label,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Package: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&pkg.name, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Safety: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    pkg.safety.display_name(),
                    Style::default()
                        .fg(safety_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    pkg.category.display_name(),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("Installed: ", Style::default().fg(Color::DarkGray)),
                if pkg.installed {
                    Span::styled("Yes ✓", Style::default().fg(Color::Green))
                } else {
                    Span::styled("No", Style::default().fg(Color::DarkGray))
                },
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Description:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                &pkg.description,
                Style::default().fg(Color::White),
            )),
        ];

        // Add warning for danger packages
        if pkg.safety == SafetyRating::Danger {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "⚠️ WARNING: This is a critical system package!",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "Removing it may brick your device.",
                Style::default().fg(Color::Red),
            )));
        }

        let details = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

        f.render_widget(details, area);
    }

    /// Draw the status bar
    fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
        let browser = &app.state.browser;

        let mode_indicator = match browser.selection_mode {
            SelectionMode::Single => Span::styled(
                " BROWSE ",
                Style::default().fg(Color::Black).bg(Color::Cyan),
            ),
            SelectionMode::Multi => Span::styled(
                " SELECT ",
                Style::default().fg(Color::Black).bg(Color::Green),
            ),
        };

        let status = browser
            .status
            .as_ref()
            .map(|s| Span::styled(format!(" {} ", s), Style::default().fg(Color::Green)))
            .unwrap_or_else(|| Span::raw(""));

        let dry_run = if app.state.dry_run {
            Span::styled(
                " 🧪 DRY RUN ",
                Style::default().fg(Color::Black).bg(Color::Yellow),
            )
        } else {
            Span::raw("")
        };

        let help = Span::styled(
            " ↑/↓: Navigate | /: Search | Space: Select | Enter: Action | ESC: Main Menu | q: Main Menu | ?: Help ",
            Style::default().fg(Color::DarkGray),
        );

        let status_bar = Paragraph::new(Line::from(vec![mode_indicator, dry_run, status, help]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(status_bar, area);
    }
}
