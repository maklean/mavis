use rand::Rng;

use crate::{algorithm::{Algorithm, AlgorithmData, AlgorithmResult, AlgorithmType}, utils::Coordinate};

pub struct NoiseMap;

const NOISE_MAP_WALL_CHANCE: i32 = 30; // in %

impl Algorithm for NoiseMap {
    fn name(&self) -> &'static str {
        "Noise Map"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Maze
    }

    fn run(&self, data: AlgorithmData) -> AlgorithmResult {
        let grid = data.grid;

        let mut rng = rand::rng();
        let mut final_path: Vec<Coordinate> = Vec::new();

        let (w, h) = (grid.len(), grid[0].len());

        for r in 0..w {
            for c in 0..h {
                let coord = (c as u16, r as u16);

                if rng.random_range(1..=100) <= NOISE_MAP_WALL_CHANCE {
                    final_path.push(coord)
                }
            }
        }

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path)
    }
}