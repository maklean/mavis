use crate::{algorithm::{Algorithm, AlgorithmResult, AlgorithmType}, grid::GridNode, utils::{self, Coordinate, manhattan_distance}};
use std::{cmp::Ordering, collections::{HashMap, HashSet, BinaryHeap}, u16};

pub struct AStar;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct AStarNode {
    coord: Coordinate,
    g_cost: u16,
    h_cost: u16,
    f_cost: u16,
}

impl AStarNode {
    fn new(coord: Coordinate, g_cost: u16, h_cost: u16) -> Self {
        Self {
            coord,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
        }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare for lower f_cost, then lower h_cost
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Algorithm for AStar {
    fn name(&self) -> &'static str {
        "A*"
    }

    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::Pathfinding
    }

    fn run(&self, grid: &Vec<Vec<GridNode>>, endpoints: Option<(Coordinate, Coordinate)>) -> AlgorithmResult {
        let (start, end) = endpoints.expect("There should be endpoints passed to A*.");
        let (w, h): (u16, u16) = (grid[0].len() as u16, grid.len() as u16);

        let mut final_path: Vec<Coordinate> = Vec::new();

        let mut open: BinaryHeap<AStarNode> = BinaryHeap::new();
        let mut closed: HashSet<Coordinate> = HashSet::new();
        let mut parent: HashMap<Coordinate, Coordinate> = HashMap::new();

        open.push(AStarNode::new(start, 0, manhattan_distance(start, end)));

        // in case no final path could be made
        let mut nearest = start;
        let mut nearest_dist: f64 = f64::INFINITY;

        while !open.is_empty() {
            let current = open.pop().expect("Should get AStarNode from non-empty BinaryHeap.");

            /*
                we can skip duplicate coordinates since the Heap already sorts for the
                lowest f-cost. Since the h-cost (heuristic estimate) doesn't change, a
                duplicate node must mean a higher g-cost, which means a longer path to
                reach this current node.

                so, we might as well just skip this one. 
            */
            if closed.contains(&current.coord) {
                continue;
            }

            closed.insert(current.coord);

            // reached end node
            if current.coord == end {
                break;
            }

            let dist_from_end = utils::euclidean_distance(current.coord, end);
            if dist_from_end < nearest_dist {
                nearest_dist = dist_from_end;
                nearest = current.coord;
            }

            // visit all neighboring nodes
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions {
                let n_x = current.coord.0 as i32 + dx;
                let n_y = current.coord.1 as i32 + dy;

                // skip if it's an out-of-bounds or wall node
                if n_x < 0 || n_x >= w as i32 || n_y < 0 || n_y >= h as i32 || grid[n_y as usize][n_x as usize] == GridNode::Wall {
                    continue;
                }

                let n_coord = (n_x as u16, n_y as u16);

                if closed.contains(&n_coord) {
                    continue;
                }

                // get g-cost from BinaryHeap entry or default to u16::MAX
                let neighbor_g_cost = open
                    .iter()
                    .find(|node| node.coord == n_coord)
                    .map_or(u16::MAX, |node| node.g_cost);

                let tentative_g = current.g_cost + 1;
                if tentative_g < neighbor_g_cost {
                    let new_neighbor_node = AStarNode::new(n_coord, tentative_g, manhattan_distance(n_coord, end));
                    parent.insert(n_coord, current.coord);
                    open.push(new_neighbor_node);
                }
            }
        }

        let mut current_node = if parent.contains_key(&end) { 
            end 
        } else { 
            // if there is no path, use nearest node to end node
            nearest
        };

        final_path.push(current_node);
        while let Some(next) = parent.get(&current_node) {
            final_path.push(*next);
            current_node = *next;
        }

        final_path.reverse();

        AlgorithmResult::new(self.name(), self.algorithm_type(), final_path)
    }
}