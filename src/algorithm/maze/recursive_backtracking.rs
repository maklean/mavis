use std::collections::HashSet;

use rand::{seq::SliceRandom};

use crate::{algorithm::{Algorithm, AlgorithmData, AlgorithmResult, AlgorithmType}, utils::Coordinate};

pub struct RecursiveBacktracking;

impl RecursiveBacktracking {
    fn recurse((cx, cy): Coordinate, visited: &mut HashSet<Coordinate>, w: i32, h: i32) {
        // return early if we've already visited this cell.
        if visited.contains(&(cx, cy)) {
            return;
        }

        visited.insert((cx, cy));

        let mut rng = rand::rng();
        let mut directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        directions.shuffle(&mut rng);

        // try to visit all neighboring cells in unbiased order
        for (dx, dy) in directions {
            let (wx, wy) = (cx as i32 + dx, cy as i32 + dy); // coordinate of wall node between current node and neighbor node
            let (nx, ny) = (wx + dx, wy + dy); // neighbor node

            // out of bounds
            if nx < 0 || nx >= w || ny < 0 || ny >= h {
                continue;
            }

            let neighbor = (nx as u16, ny as u16);
            let wall = (wx as u16, wy as u16);

            // we've already visited this node, backtrack.
            if visited.contains(&neighbor) {
                continue;
            }

            // carve wall between current node and visited node, then explore neighbor node
            visited.insert(wall);
            RecursiveBacktracking::recurse(neighbor, visited, w, h);
        }
    }
}

impl Algorithm for RecursiveBacktracking {
    fn name(&self) -> &'static str {
        "Recursive Backtracking"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Maze
    }

    fn run(&self, data: AlgorithmData) -> AlgorithmResult {
        let grid = data.grid;
        let mut final_path: HashSet<Coordinate> = HashSet::new(); // hashset for O(1) removal (also, since they aren't ordered it gives a pretty cool generation effect)
        let (w, h) = (grid[0].len(), grid.len());

        // fill entire grid with wall nodes
        for r in 0..h as u16 {
            for c in 0..w as u16 {
                final_path.insert((c, r));
            }
        }

        let mut visited: HashSet<Coordinate> = HashSet::new();
        RecursiveBacktracking::recurse((1, 1), &mut visited, w as i32, h as i32); // start exploring at odd node

        // remove visited cells from final path
        final_path.retain(|c| !visited.contains(c));
        
        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path.into_iter().collect())
    }
}