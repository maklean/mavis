use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use crate::utils;

pub enum Event {
    AppQuit,
    MouseDown(utils::Coordinate)
}

pub fn app_event_loop(tx: mpsc::Sender<Event>) -> io::Result<()> {
    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(event) => {
                if let Some(result) = handle_key_event(event) {
                    tx.send(result).expect("Should be able to send key event");
                }
            },
            crossterm::event::Event::Mouse(event) => {
                if let Some(result) = handle_mouse_event(event) {
                    tx.send(result).expect("Should be able to send mouse event");
                }
            }
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

fn handle_mouse_event(event: MouseEvent) -> Option<Event> {
    if let MouseEventKind::Down(button) = event.kind {
        if button != MouseButton::Left {
            return None;
        }

        return Some(Event::MouseDown((event.column, event.row)));
    }

    None
}