use std::{io, sync::mpsc};

use crate::{algorithm::AlgorithmType, event::{self}, grid::{Grid, GridNode}, sidebar::Sidebar, ui::{self}};

pub struct App {
    pub grid: Grid, // the grid instance of the app
    pub sidebar: Sidebar, // the sidebar instance of the app
}

impl App {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            sidebar: Sidebar::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal, rx: mpsc::Receiver<event::Event>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::render(f, self))?;

            // handle user inputs
            if let Ok(event) = rx.try_recv() {
                match event {
                    event::Event::AppQuit => break,
                    event::Event::MouseDown(position) => {
                        if !self.grid.is_position_out_of_bounds(position) {
                            // do stuff
                            break;
                        }
                    }
                    event::Event::ScrollUp => self.sidebar.prev(),
                    event::Event::ScrollDown => self.sidebar.next(),
                    event::Event::Select => self.sidebar.select(&mut self.grid),
                }
            }

            // handle current algorithm if there is one
            self.handle_algorithm();
        }

        Ok(())
    }

    fn handle_algorithm(&mut self) {
        // the borrow checker had me believing I had a clean solution until I had to pull up clone()...
        let Some(algorithm) = self.grid.algorithm.clone() else {
            return;
        };

        let mut current_index = algorithm.current_index;

        // if we're at the start, we should reset the entire map for Maze generation algorithms
        if algorithm.algorithm_type == AlgorithmType::Maze && current_index == 0 {
            self.grid.reset(None);
        }

        // do 20 steps at a time
        for _ in 0..20 {
            let (r, c) = algorithm.final_path[current_index];
            self.grid.nodes[r as usize][c as usize] = if algorithm.algorithm_type == AlgorithmType::Maze { GridNode::Wall } else { GridNode::Path };
            current_index += 1;

            if current_index >= algorithm.final_path.len() {
                self.grid.algorithm = None;
                return;
            }
        }

        // :(
        self.grid.algorithm.as_mut().unwrap().current_index = current_index;
    }
}