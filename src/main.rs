use std::{io::{self}, sync::mpsc, thread};

use crate::{app::App, event::app_event_loop};

mod app;
mod grid;
mod sidebar;
mod ui;
mod event;
mod utils;
mod algorithm;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    // enable mouse detection
    crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;

    // startup event-loop threads
    let (tx, rx) = mpsc::channel::<event::Event>();

    // spawn thread for key handling
    let key_tx = tx.clone();
    thread::spawn(|| {
        app_event_loop(key_tx).unwrap();
    });

    let mut app = App::new();
    match app.run(&mut terminal, rx) {
        Ok(_) | Err(_) => {
            // close application
            ratatui::restore();
            crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
        }
    }

    Ok(())
}