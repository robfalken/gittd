use crossterm::event::{Event, KeyCode};

pub enum KeyboardEvent {
    Number(usize),
    Q,
    D,
    Ignore,
}

impl KeyboardEvent {
    pub fn from_event(event: Event) -> Self {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    '0'..='9' => KeyboardEvent::Number(c.to_digit(10).unwrap() as usize),
                    'q' => KeyboardEvent::Q,
                    'd' => KeyboardEvent::D,
                    _ => KeyboardEvent::Ignore,
                },
                _ => KeyboardEvent::Ignore,
            },
            _ => KeyboardEvent::Ignore,
        }
    }
}

