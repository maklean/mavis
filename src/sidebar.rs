use std::rc::Rc;

use ratatui::widgets::ListState;

use crate::{algorithm::{Algorithm, AlgorithmData, maze::{noise_map::NoiseMap, prims_algorithm::PrimsAlgorithm, recursive_backtracking::RecursiveBacktracking}, pathfinding::{a_star::AStar, breadth_first_search::BreadthFirstSearch, depth_first_search::DepthFirstSearch}}, grid};

pub struct Sidebar {
    pub page: SidebarPage, // the current page the sidebar is in
    pub state: ListState, // the state (position) of the sidebar
}

impl Sidebar {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            page: SidebarPage::Main,
            state
        }
    }

    pub fn next(&mut self) {
        if let Some(idx) = self.state.selected() {
            self.state.select(Some((idx+1) % self.page.options().len()));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn prev(&mut self) {
        if let Some(idx) = self.state.selected() {
            // yuck
            self.state.select(Some(((idx as i32)-1 % (self.page.options().len() as i32)) as usize));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn select(&mut self, grid: &mut grid::Grid) {
        let index = self.state.selected().unwrap_or(0);
        let action = &self.page.options()[index].action;

        match action {
            SidebarAction::SwitchPage(page) => {
                self.state.select(Some(0));
                self.page = page.clone();
            },

            SidebarAction::RunAlgorithm(algorithm) => {
                let result = algorithm.as_ref().run(AlgorithmData::new(&grid.nodes, None));
                grid.algorithm = Some(result);
            },

            SidebarAction::RunMarkersState(algorithm) => {
                grid.markers_state.is_placing = true;
                grid.markers_state.target_algorithm = Some(algorithm.clone());
            }
        }
    }
}

#[derive(Clone)]
pub enum SidebarPage {
    Main,
    MazeAlgorithms,
    PathfindingAlgorithms
}

impl SidebarPage {
    pub fn options(&self) -> Vec<SidebarOption> {
        let back_to_home = SidebarOption::new("Back", SidebarAction::SwitchPage(SidebarPage::Main));

        match self {
            SidebarPage::Main => vec![
                SidebarOption::new("View Maze Algorithms", SidebarAction::SwitchPage(SidebarPage::MazeAlgorithms)),
                SidebarOption::new("View Pathfinding Algorithms", SidebarAction::SwitchPage(SidebarPage::PathfindingAlgorithms)),
            ],
            SidebarPage::MazeAlgorithms => vec![
                SidebarOption::new("Noise Map", SidebarAction::RunAlgorithm(Box::new(NoiseMap))),
                SidebarOption::new("Recursive Backtracking", SidebarAction::RunAlgorithm(Box::new(RecursiveBacktracking))),
                SidebarOption::new("Prim's Algorithm", SidebarAction::RunAlgorithm(Box::new(PrimsAlgorithm))),
                back_to_home,
            ],
            SidebarPage::PathfindingAlgorithms => vec![
                SidebarOption::new("BFS", SidebarAction::RunMarkersState(Rc::new(BreadthFirstSearch))),
                SidebarOption::new("A*", SidebarAction::RunMarkersState(Rc::new(AStar))),
                SidebarOption::new("DFS", SidebarAction::RunMarkersState(Rc::new(DepthFirstSearch))),
                back_to_home,
            ]
        }
    }
}

pub struct SidebarOption {
    pub title: &'static str,
    action: SidebarAction
}

impl SidebarOption {
    fn new(title: &'static str, action: SidebarAction) -> Self {
        Self {
            title,
            action
        }
    }
}

enum SidebarAction {
    SwitchPage(SidebarPage),
    RunAlgorithm(Box<dyn Algorithm>),
    RunMarkersState(Rc<dyn Algorithm>),
}