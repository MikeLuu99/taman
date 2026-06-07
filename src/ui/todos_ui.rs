use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_todos(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = if app.todo_editing {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(0)])
            .split(area)
    };

    // Todo list
    let list_block = Block::default()
        .title_top(
            Line::from(" Todos ")
                .style(Style::default().fg(app.theme.blocks))
                .centered(),
        )
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));

    let items: Vec<ListItem> = if app.todos.is_empty() {
        vec![ListItem::new("  No todos yet. Press [N] to add one.")
            .style(Style::default().fg(app.theme.secondary_text))]
    } else {
        app.todos
            .iter()
            .enumerate()
            .map(|(i, todo)| {
                let is_selected = i == app.todos_selected;
                let checkbox = if todo.completed { "[x]" } else { "[ ]" };
                let prefix = if is_selected { "→" } else { " " };
                let text = format!("{} {} {}", prefix, checkbox, todo.title);
                let style = if todo.completed {
                    Style::default().fg(app.theme.secondary_text)
                } else if is_selected {
                    Style::default()
                        .fg(app.theme.highlight)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                ListItem::new(text).style(style)
            })
            .collect()
    };

    let list = List::new(items).block(list_block);
    f.render_widget(list, chunks[0]);

    // Input bar (only when editing)
    if app.todo_editing {
        let input_block = Block::default()
            .title_top(
                Line::from(" New Todo ")
                    .style(Style::default().fg(app.theme.highlight))
                    .centered(),
            )
            .borders(Borders::ALL)
            .style(Style::default().fg(app.theme.highlight));
        let input_text = format!("{}_", app.todo_input);
        let input = Paragraph::new(input_text)
            .block(input_block)
            .style(Style::default().fg(app.theme.text));
        f.render_widget(input, chunks[1]);
    }
}
