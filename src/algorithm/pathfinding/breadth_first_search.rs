use crate::{algorithm::{Algorithm, AlgorithmResult, AlgorithmType}, grid::GridNode, utils::Coordinate};
use std::collections::{VecDeque, HashMap};

pub struct BreadthFirstSearch;

impl Algorithm for BreadthFirstSearch {
    fn name(&self) -> &'static str {
        "Breadth First Search"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Pathfinding
    }

    fn run(&self, grid: &Vec<Vec<GridNode>>, endpoints: Option<(Coordinate, Coordinate)>) -> AlgorithmResult {
        let (start, end) = endpoints.expect("There should be endpoints passed to BFS");

        let mut final_path: Vec<Coordinate> = Vec::new();
        let (w, h): (u16, u16) = (grid[0].len() as u16, grid.len() as u16);

        let mut queue: VecDeque<Coordinate> = VecDeque::new();
        let mut distances: HashMap<Coordinate, i32> = HashMap::new();
        let mut parents: HashMap<Coordinate, Coordinate> = HashMap::new();

        // set distance of source node to 0
        distances.insert(start, 0);
        queue.push_front(start);

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == end { break; }

            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in directions.iter() {
                let n_x = x as i32 + dx;
                let n_y = y as i32 + dy;

                // Correct bounds check
                if n_x >= 0 && n_x < w as i32 && n_y >= 0 && n_y < h as i32 {
                    let n_coord = (n_x as u16, n_y as u16);

                    if grid[n_coord.1 as usize][n_coord.0 as usize] == GridNode::Wall {
                        continue;
                    }

                    if !distances.contains_key(&n_coord) {
                        parents.insert(n_coord, (x, y));
                        // Distance should increment
                        let d = *distances.get(&(x, y)).unwrap();
                        distances.insert(n_coord, d + 1);
                        
                        queue.push_back(n_coord);
                    }
                }
            }
        }

        if !distances.contains_key(&end) {
            return AlgorithmResult::new(self.name(), self.algorithm_type(), final_path);
        }

        final_path.push(end);
        let mut current_node = end;

        while parents.contains_key(&current_node) {
            let next_node = *parents.get(&current_node).expect("Should be able to get existing value");
            final_path.push(next_node);

            current_node = next_node;
        }

        final_path.reverse();

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path)
    }
}