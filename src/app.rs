use std::{io, sync::mpsc};

use crate::{algorithm::{AlgorithmData, AlgorithmResultStatus, AlgorithmType, FrameNodeKind}, event::{self}, grid::{Grid, GridNode}, sidebar::Sidebar, ui::{self}, utils::Coordinate};

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
                    event::Event::MouseDown(position) => self.handle_marker_placement(position),
                    event::Event::ScrollUp => self.sidebar.prev(),
                    event::Event::ScrollDown => self.sidebar.next(),
                    event::Event::Select => self.sidebar.select(&mut self.grid),
                }
            }

            // handle current algorithm if there is one
            if let Some(algorithm) = self.grid.algorithm.as_ref() {
                if algorithm.status == AlgorithmResultStatus::Frames {
                    self.handle_frames();
                } else {
                    self.handle_algorithm();
                }
            }
        }

        Ok(())
    }

    fn handle_algorithm(&mut self) {
        // the borrow checker had me believing I had a clean solution until I had to pull up clone()...
        let Some(algorithm) = self.grid.algorithm.clone() else {
            return;
        };

        let mut current_index = algorithm.current_index;

        if algorithm.final_path.len() == 0 {
            self.grid.algorithm = None;
            return;
        }

        if current_index == 0 {
            if algorithm.algorithm_type == AlgorithmType::Maze {
                // if we're at the start, we should reset the entire map for Maze generation algorithms
                self.grid.reset(None);
            } else {
                self.grid.nodes = self.grid.nodes.iter().map(|row| {
                let new_row: Vec<GridNode> = row
                    .iter()
                    .map(|n| if *n == GridNode::Wall { GridNode::Wall } else { GridNode::Empty })
                    .collect();

                    new_row
                }).collect();
            }
        }

        let steps = if algorithm.algorithm_type == AlgorithmType::Maze { 20 } else { 3 };

        for _ in 0..steps {
            let (c, r) = algorithm.final_path[current_index];
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

    fn handle_frames(&mut self) {
        let Some(algorithm) = self.grid.algorithm.clone() else { return };

        if algorithm.frames.len() == 0 {
            self.grid.algorithm.as_mut().unwrap().current_index = 0;
            self.grid.algorithm.as_mut().unwrap().status = AlgorithmResultStatus::FinalPath;
            return;
        }

        let mut current_index = algorithm.current_index;

        if current_index == 0 {
            if algorithm.algorithm_type == AlgorithmType::Maze {
                // if we're at the start, we should reset the entire map for Maze generation algorithms
                self.grid.reset(None);
            } else {
                self.grid.nodes = self.grid.nodes.iter().map(|row| {
                let new_row: Vec<GridNode> = row
                    .iter()
                    .map(|n| if *n == GridNode::Wall { GridNode::Wall } else { GridNode::Empty })
                    .collect();

                    new_row
                }).collect();
            }
        }

        let steps = if algorithm.algorithm_type == AlgorithmType::Maze { 60 } else { 9 };

        for _ in 0..steps {
            let (c, r) = algorithm.frames[current_index].coord;
            let kind = &algorithm.frames[current_index].kind;

            self.grid.nodes[r as usize][c as usize] = match kind {
                FrameNodeKind::EXPLORED => if algorithm.algorithm_type == AlgorithmType::Maze { GridNode::ExploredWall } else { GridNode::ExploredPath },
                FrameNodeKind::PENDING => if algorithm.algorithm_type == AlgorithmType::Maze { GridNode::PendingWall } else { GridNode::PendingPath },
            };

            current_index += 1;

            if current_index >= algorithm.frames.len() {
                self.grid.algorithm.as_mut().unwrap().current_index = 0;
                self.grid.algorithm.as_mut().unwrap().status = AlgorithmResultStatus::FinalPath;
                return;
            }
        }

        self.grid.algorithm.as_mut().unwrap().current_index = current_index;
    }

    fn handle_marker_placement(&mut self, position: Coordinate) {
        if self.grid.markers_state.is_placing && !self.grid.is_position_out_of_bounds(position) {
            let (grid_c, grid_r) = (position.0 - self.grid.bounds.0.0, position.1 - self.grid.bounds.0.1);

            if self.grid.nodes[grid_r as usize][grid_c as usize] == GridNode::Wall {
                return;
            }

            *(self.grid.markers_state.next()) = Some((grid_c, grid_r));

            if self.grid.markers_state.second != None {
                let target_algorithm = self.grid.markers_state.target_algorithm.as_ref().unwrap();
                let endpoints = (self.grid.markers_state.first.unwrap(), self.grid.markers_state.second.unwrap());

                let result = target_algorithm.run(AlgorithmData::new(&self.grid.nodes, Some(endpoints)));

                self.grid.algorithm = Some(result);
                
                self.grid.markers_state.reset();
            }
        }
    }
}