use ratatui::widgets::ListState;

use crate::grid;

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

    pub fn select(&mut self) {
        self.state.select(Some(0));

        if let Some(idx) = self.state.selected() {
            match &self.page.options()[idx].action {
                SidebarAction::SwitchPage(page) => self.page = page.clone()
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
                back_to_home,
            ],
            SidebarPage::PathfindingAlgorithms => vec![
                back_to_home
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
    SwitchPage(SidebarPage)
}