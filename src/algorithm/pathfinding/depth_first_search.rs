use crate::{algorithm::{Algorithm, AlgorithmResult, AlgorithmType}, grid::GridNode, utils::{self, Coordinate}};
use std::collections::{HashMap, HashSet};

pub struct DepthFirstSearch;

impl DepthFirstSearch {
    // Recursively traverses the grid, returns the nearest node to the end node and its distance from the end node.
    fn recurse(coord: Coordinate, visited: &mut HashSet<Coordinate>, parents: &mut HashMap<Coordinate, Coordinate>, grid: &Vec<Vec<GridNode>>, target: Coordinate) -> (Coordinate, f64) {
        visited.insert(coord);

        if coord == target {
            return (coord, 0.0);
        }
        
        // in case no final path could be made
        let mut nearest = coord;
        let mut nearest_dist: f64 = utils::euclidean_distance(coord, target);

        let (w, h) = (grid[0].len() as i32, grid.len() as i32);
        
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let (nx, ny) = (coord.0 as i32 + dx, coord.1 as i32 + dy);

            // skip out-of-bounds or wall node
            if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[ny as usize][nx as usize] == GridNode::Wall {
                continue;
            }

            let neighbor = (nx as u16, ny as u16);

            // skip visited nodes
            if visited.contains(&neighbor) {
                continue;
            }

            parents.insert(neighbor, coord);

            let (candidate, dist) = DepthFirstSearch::recurse(neighbor, visited, parents, grid, target);
            if nearest_dist < dist {
                nearest = candidate;
                nearest_dist = dist;
            }
        }

        (nearest, nearest_dist)
    }
}

impl Algorithm for DepthFirstSearch {
    fn name(&self) -> &'static str {
        "Depth-First Search"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Pathfinding
    }

    fn run(&self, grid: &Vec<Vec<GridNode>>, endpoints: Option<(Coordinate, Coordinate)>) -> AlgorithmResult {
        let (start, end) = endpoints.expect("There should be endpoints passed to Depth-First Search.");

        let mut final_path: Vec<Coordinate> = Vec::new();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut parents: HashMap<Coordinate, Coordinate> = HashMap::new();

        visited.insert(start);

        let (nearest, _) = DepthFirstSearch::recurse(start, &mut visited, &mut parents, grid, end);
        let mut current_node = *visited.get(&end).unwrap_or(&nearest);

        final_path.push(current_node);

        while parents.contains_key(&current_node) {
            let next_node = *parents.get(&current_node).expect("Should be able to get existing value");
            final_path.push(next_node);

            current_node = next_node;
        }

        final_path.reverse();

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path)
    }
}