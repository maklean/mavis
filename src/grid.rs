use std::rc::Rc;

use ratatui::{style::{Color, Style}, text::Span};

use crate::{algorithm::{Algorithm, AlgorithmResult}, utils::Coordinate};

#[derive(PartialEq)]
pub enum GridNode {
    Empty,
    Wall,
    Path,
    ExploredPath,
    PendingPath,
    ExploredWall,
    PendingWall,
}

impl GridNode {
    pub fn span(&self) -> Span<'_> {
        match self {
            GridNode::Empty => Span::styled(" ", Style::default().fg(Color::White)),
            GridNode::Wall => Span::styled("█", Style::default().fg(Color::White)),
            GridNode::Path => Span::styled("@", Style::default().fg(Color::LightGreen)),
            GridNode::ExploredPath => Span::styled("@", Style::default().fg(Color::Gray)),
            GridNode::PendingPath => Span::styled("@", Style::default().fg(Color::LightBlue)),
            GridNode::ExploredWall => Span::styled("█", Style::default().fg(Color::LightYellow)),
            GridNode::PendingWall => Span::styled("█", Style::default().fg(Color::LightBlue)),
        }
    }
}

pub struct MarkersState {
    pub is_placing: bool,
    pub first: Option<Coordinate>,
    pub second: Option<Coordinate>,
    pub target_algorithm: Option<Rc<dyn Algorithm>>
}

impl MarkersState {
    fn new(target_algorithm: Option<Rc<dyn Algorithm>>) -> Self {
        Self {
            is_placing: false,
            first: None,
            second: None,
            target_algorithm: target_algorithm
        }
    }

    pub fn reset(&mut self) {
        self.is_placing = false;
        self.first = None;
        self.second = None;
        self.target_algorithm = None;
    }

    pub fn next(&mut self) -> &mut Option<Coordinate> {
        if !self.is_placing {
            panic!("called MarkersState::next() when no placing is happening");
        }

        if self.first == None {
            &mut self.first
        } else {
            &mut self.second
        }
    }
}

pub struct Grid {
    pub nodes: Vec<Vec<GridNode>>,
    pub bounds: (Coordinate, Coordinate),
    pub algorithm: Option<AlgorithmResult>,
    pub markers_state: MarkersState,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            bounds: ((0, 0), (0, 0)),
            algorithm: None,
            markers_state: MarkersState::new(None)
        }
    }

    pub fn reset(&mut self, new_size: Option<(u16, u16)>) {
        let (width, height) = new_size.unwrap_or((self.width(), self.height()));

        self.nodes = (0..height).map(|_| {
            (0..width).map(|_| GridNode::Empty).collect()
        }).collect();
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