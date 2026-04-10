use rand::Rng;

use crate::{algorithm::{Algorithm, AlgorithmResult, AlgorithmType}, grid::GridNode, utils::Coordinate};

pub struct NoiseMap;

const NOISE_MAP_WALL_CHANCE: i32 = 10; // in %

impl Algorithm for NoiseMap {
    fn name(&self) -> &'static str {
        "Noise Map"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Maze
    }

    fn run(&self, grid: &Vec<Vec<GridNode>>) -> AlgorithmResult {
        let mut rng = rand::rng();
        let mut final_path: Vec<Coordinate> = Vec::new();

        let (w, h) = (grid.len(), grid[0].len());

        for r in 0..w {
            for c in 0..h {
                let coord = (r as u16, c as u16);

                if rng.random_range(1..=100) <= NOISE_MAP_WALL_CHANCE {
                    final_path.push(coord)
                }
            }
        }

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path)
    }
}