use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::garden::GardenEntryKind;

const CELL_WIDTH: usize = 22;

pub fn draw_garden(f: &mut Frame, app: &mut App, area: Rect) {
    let outer_block = Block::default()
        .title_top(
            Line::from(" Garden ")
                .style(Style::default().fg(app.theme.blocks))
                .centered(),
        )
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    let inner = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Summary line
    let flower_count = app.garden.entries.iter().filter(|e| matches!(e.kind, GardenEntryKind::Flower)).count();
    let plant_count = app.garden.entries.iter().filter(|e| matches!(e.kind, GardenEntryKind::Plant)).count();
    let summary = format!("🌸 {} flowers   🪴 {} plants   — {} total", flower_count, plant_count, app.garden.entries.len());
    let summary_para = Paragraph::new(summary)
        .style(Style::default().fg(app.theme.secondary_text).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(summary_para, chunks[0]);

    if app.garden.entries.is_empty() {
        let placeholder = Paragraph::new("No items yet. Complete todos or grow plants!")
            .style(Style::default().fg(app.theme.secondary_text))
            .alignment(Alignment::Center);
        f.render_widget(placeholder, chunks[1]);
        return;
    }

    let cols = ((chunks[1].width as usize) / CELL_WIDTH).max(1);
    let entries = &app.garden.entries;

    // Build all grid rows (2 lines each: emoji, blank)
    let mut all_rows: Vec<[Line; 2]> = vec![];
    for chunk in entries.chunks(cols) {
        let mut emoji_spans: Vec<Span> = vec![];

        for entry in chunk {
            let color = match entry.kind {
                GardenEntryKind::Flower => app.theme.rose,
                GardenEntryKind::Plant => app.theme.gauge_finished,
            };
            let emoji_cell = format!("{:<width$}", format!("  {} ", entry.emoji), width = CELL_WIDTH);
            emoji_spans.push(Span::styled(emoji_cell, Style::default().fg(color)));
        }

        all_rows.push([
            Line::from(emoji_spans),
            Line::from(""),
        ]);
    }

    // Clamp scroll
    let visible_rows = (chunks[1].height as usize / 2).max(1);
    let max_scroll = all_rows.len().saturating_sub(visible_rows);
    if app.garden_scroll > max_scroll {
        app.garden_scroll = max_scroll;
    }

    let visible: Vec<Line> = all_rows
        .into_iter()
        .skip(app.garden_scroll)
        .take(visible_rows)
        .flat_map(|[a, b]| [a, b])
        .collect();

    let grid = Paragraph::new(visible);
    f.render_widget(grid, chunks[1]);
}
