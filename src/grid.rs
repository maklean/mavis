use crate::utils::Coordinate;

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
    pub nodes: Vec<Vec<GridNode>>,
    pub bounds: (Coordinate, Coordinate)
}

impl Grid {
    pub fn new() -> Self {
        Self {
            state: GridState::Idle,
            nodes: Vec::new(),
            bounds: ((0, 0), (0, 0))
        }
    }

    pub fn is_position_out_of_bounds(&self, position: Coordinate) -> bool {
        let ((start_x, start_y), (end_x, end_y)) = self.bounds;
        let (pos_x, pos_y) = position;

        pos_x < start_x || pos_x > end_x || pos_y < start_y || pos_y > end_y
    }

    pub fn width(&self) -> u16 {
        if self.height() == 0 {
            return 0;
        }
        
        self.nodes[0].len() as u16
    }

    pub fn height(&self) -> u16 {
        self.nodes.len() as u16
    }
}