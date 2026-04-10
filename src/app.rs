use std::{io, sync::mpsc};

use crate::{grid::Grid, sidebar::Sidebar, ui::{self}, event::{self}};

pub struct App {
    grid: Grid, // the grid instance of the app
    sidebar: Sidebar, // the sidebar instance of the app
}

impl App {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            sidebar: Sidebar {}
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal, rx: mpsc::Receiver<event::Event>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::render(f, &mut self.grid))?;

            // handle user inputs
            if let Ok(event) = rx.try_recv() {
                match event {
                    event::Event::AppQuit => {
                        break;
                    },
                    event::Event::MouseDown(position) => {
                        if !self.grid.is_position_out_of_bounds(position) {
                            // do stuff
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}