use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy)]
pub enum InputAction {
    Tab(u8), // 1-5
    Left,
    Right,
    Up,
    Down,
    Space,
    Stop,
    Quit,
    Enter,
    Delete,
    NewTodo,
    EditTodo,
    Escape,
}

pub fn handle_key(key: KeyEvent) -> Option<InputAction> {
    match key.code {
        KeyCode::Char('1') => Some(InputAction::Tab(1)),
        KeyCode::Char('2') => Some(InputAction::Tab(2)),
        KeyCode::Char('3') => Some(InputAction::Tab(3)),
        KeyCode::Char('4') => Some(InputAction::Tab(4)),
        KeyCode::Char('5') => Some(InputAction::Tab(5)),
        KeyCode::Char('6') => Some(InputAction::Tab(6)),
        KeyCode::Left => Some(InputAction::Left),
        KeyCode::Right => Some(InputAction::Right),
        KeyCode::Up => Some(InputAction::Up),
        KeyCode::Down => Some(InputAction::Down),
        KeyCode::Char(' ') => Some(InputAction::Space),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(InputAction::Stop),
        KeyCode::Char('n') | KeyCode::Char('N') => Some(InputAction::NewTodo),
        KeyCode::Char('e') | KeyCode::Char('E') => Some(InputAction::EditTodo),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(InputAction::Quit),
        KeyCode::Enter => Some(InputAction::Enter),
        KeyCode::Delete => Some(InputAction::Delete),
        KeyCode::Backspace => Some(InputAction::Delete),
        KeyCode::Esc => Some(InputAction::Escape),
        _ => None,
    }
}