use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use crate::{
    solution::Solution,
    vector::{Vec2, RIGHT},
    vector_map::VectorMap,
};

pub struct Day16;

impl Solution for Day16 {
    type Answer = i64;
    fn day(&self) -> u8 {
        16
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let maze = input.parse::<Maze>()?;
        Ok(maze.find_shortest_path()[0].cost)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let maze = input.parse::<Maze>()?;
        let paths = maze.find_shortest_path();
        let tiles = paths
            .into_iter()
            .flat_map(|p| p.visited_tiles)
            .collect::<HashSet<_>>();

        Ok(tiles.len() as i64)
    }
}

struct Maze {
    map: VectorMap<bool>,
    target: Vec2,
    position: Vec2,
    direction: Vec2,
}

impl Maze {
    fn find_shortest_path(&self) -> Vec<Move> {
        let mut p_queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut optimal_cost = i64::MAX;
        let mut paths = Vec::new();
        p_queue.push(Move {
            position: self.position,
            direction: self.direction,
            cost: 0,
            visited_tiles: vec![self.position],
        });

        while let Some(current_move) = p_queue.pop() {
            // End condition
            if current_move.position == self.target {
                if current_move.cost > optimal_cost {
                    break;
                }
                optimal_cost = current_move.cost;
                paths.push(current_move.clone());
            }

            // I'm on a wall
            match self.map.get(&current_move.position) {
                Some(true) | None => continue,
                _ => {}
            }

            if let Some(&cost) = visited.get(&(current_move.position, current_move.direction)) {
                if cost < current_move.cost {
                    continue;
                }
            }

            visited.insert(
                (current_move.position, current_move.direction),
                current_move.cost,
            );

            let mut new_visited_tiles = current_move.visited_tiles.clone();
            new_visited_tiles.push(current_move.position + current_move.direction);

            let next_moves = [
                Move {
                    position: current_move.position + current_move.direction,
                    direction: current_move.direction,
                    cost: current_move.cost + 1,
                    visited_tiles: new_visited_tiles,
                },
                Move {
                    position: current_move.position,
                    direction: current_move.direction.rotate_clockwise(),
                    cost: current_move.cost + 1000,
                    visited_tiles: current_move.visited_tiles.clone(),
                },
                Move {
                    position: current_move.position,
                    direction: current_move.direction.rotate_counter_clockwise(),
                    cost: current_move.cost + 1000,
                    visited_tiles: current_move.visited_tiles,
                },
            ];

            for next_move in next_moves.into_iter() {
                p_queue.push(next_move);
            }
        }

        paths
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut map = VectorMap::new((width, height).into(), false);
        let mut target = None;
        let mut position = None;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => map.set(&(x, y).into(), true),
                    'S' => position = Some((x, y).into()),
                    'E' => target = Some((x, y).into()),
                    _ => {}
                }
            }
        }

        Ok(Self {
            map,
            target: target.unwrap(),
            position: position.unwrap(),
            direction: RIGHT,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    position: Vec2,
    direction: Vec2,
    visited_tiles: Vec<Vec2>,
    cost: i64,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Day16.run_test1(), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Day16.run_test2(), 64);
    }
}
