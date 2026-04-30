use crate::{grid::GridNode, utils::Coordinate};

pub mod maze;
pub mod pathfinding;

pub trait Algorithm {
    fn name(&self) -> &'static str;
    fn algorithm_type(&self) -> AlgorithmType;
    fn run(&self, data: AlgorithmData) -> AlgorithmResult;
}

#[derive(PartialEq, Clone)]
pub enum AlgorithmType {
    Maze,
    Pathfinding
}

#[derive(Clone)]
pub struct AlgorithmResult {
    pub name: &'static str,
    pub algorithm_type: AlgorithmType,
    pub final_path: Vec<Coordinate>, // the final path of coordinates
    pub current_index: usize, // current index into final_path
}

impl AlgorithmResult {
    pub fn new(name: &'static str, algorithm_type: AlgorithmType, path: Vec<Coordinate>) -> Self {
        Self {
            name,
            algorithm_type,
            final_path: path,
            current_index: 0
        }
    }
}

pub struct AlgorithmData<'a> {
    grid: &'a Vec<Vec<GridNode>>,
    endpoints: Option<(Coordinate, Coordinate)>
}

impl<'a> AlgorithmData<'a> {
    pub fn new(grid: &'a Vec<Vec<GridNode>>, endpoints: Option<(Coordinate, Coordinate)>) -> Self {
        Self {
            grid: grid,
            endpoints,
        }
    }
}