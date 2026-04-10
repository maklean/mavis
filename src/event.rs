use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub enum Event {
    AppQuit,
}

pub fn app_event_loop(tx: mpsc::Sender<Event>) -> io::Result<()> {
    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(event) => {
                if let Some(result) = handle_key_event(event) {
                    tx.send(result).expect("Should be able to send result event");
                }
            },
            _ => {}
        }
    }
}

fn handle_key_event(event: KeyEvent) -> Option<Event> {
    if event.kind != KeyEventKind::Press {
        return None;
    }

    match event.code {
        KeyCode::Esc => Some(Event::AppQuit),
        _ => None
    }
}