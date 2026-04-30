use std::collections::HashSet;

use rand::Rng;

use crate::{algorithm::{Algorithm, AlgorithmData, AlgorithmResult, AlgorithmType}, utils::Coordinate};

pub struct PrimsAlgorithm;

impl PrimsAlgorithm {
    // marks the given cell as an "in" and makes its neighbors frontier cells
    fn mark(coord: Coordinate, in_cells: &mut HashSet<Coordinate>, frontier_cells: &mut HashSet<Coordinate>, w: i32, h: i32) {
        // mark current cell as an in-cell :o
        in_cells.insert(coord);

        let (cx, cy) = (coord.0 as i32, coord.1 as i32);
        let directions = vec![(0, 2), (2, 0), (0, -2), (-2, 0)]; // we should only visit the next cells with the same parity since the wall node goes between them

        for (dx, dy) in directions {
            let (x, y) = (cx + dx, cy + dy);

            // out of bounds
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }

            // add neighboring cell as frontier cell
            frontier_cells.insert((x as u16, y as u16));
        }
    }

    // returns the "in" neighbors of a given frontier cell
    fn neighbors(coord: Coordinate, in_cells: &HashSet<Coordinate>, w: i32, h: i32) -> Vec<Coordinate> {
        let mut neighbors: Vec<Coordinate> = Vec::new();

        let (cx, cy) = (coord.0 as i32, coord.1 as i32);
        let directions = vec![(0, 2), (2, 0), (0, -2), (-2, 0)];

        for (dx, dy) in directions {
            let (x, y) = (cx + dx, cy + dy);

            // out of bounds
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }

            let neighbor = (x as u16, y as u16);
            if in_cells.contains(&neighbor) {
                neighbors.push(neighbor);
            }
        }

        neighbors
    }
}

impl Algorithm for PrimsAlgorithm {
    fn name(&self) -> &'static str {
        "Prim's Algorithm"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Maze
    }

    fn run(&self, data: AlgorithmData) -> AlgorithmResult {
        let grid = data.grid;
        let mut final_path: HashSet<Coordinate> = HashSet::new(); // hashset for O(1) removal
        let (w, h) = (grid[0].len() as i32, grid.len() as i32);

        // fill entire grid with wall nodes
        for r in 0..h as u16 {
            for c in 0..w as u16 {
                final_path.insert((c, r));
            }
        }
        
        let mut in_cells: HashSet<Coordinate> = HashSet::new();
        let mut frontier_cells: HashSet<Coordinate> = HashSet::new();
        let mut rng = rand::rng();

        let (start_c, start_r) = (rng.random_range(0..w) as u16, rng.random_range(0..h) as u16);
        PrimsAlgorithm::mark((start_c, start_r), &mut in_cells, &mut frontier_cells, w, h);

        while !frontier_cells.is_empty() {
            let idx = rng.random_range(0..frontier_cells.len());
            let c_coord = *frontier_cells.iter().nth(idx).expect("There should be a frontier cell in a non-empty set.");

            frontier_cells.remove(&c_coord);

            // skip node if it's already an empty node
            if !final_path.contains(&c_coord) {
                continue;
            }

            let n = PrimsAlgorithm::neighbors(c_coord, &in_cells, w, h);
            let n_coord = n[rng.random_range(0..n.len())];

            let wx = c_coord.0 as i32 + (n_coord.0 as i32 - c_coord.0 as i32) / 2;
            let wy = c_coord.1 as i32 + (n_coord.1 as i32 - c_coord.1 as i32) / 2;

            let w_coord = (wx as u16, wy as u16);

            // remove nodes
            final_path.remove(&w_coord);
            final_path.remove(&c_coord);
            final_path.remove(&n_coord);

            // mark frontier cell
            PrimsAlgorithm::mark(c_coord, &mut in_cells, &mut frontier_cells, w, h);
        }

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path.into_iter().collect())
    }
}