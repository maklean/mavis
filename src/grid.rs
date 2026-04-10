pub enum GridNode {
    Empty,
    Wall,
    Path
}

pub enum GridState {
    Idle
}

pub struct Grid {
    pub state: GridState,
    pub nodes: Vec<Vec<GridNode>>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            state: GridState::Idle,
            nodes: Vec::new()
        }
    }
}