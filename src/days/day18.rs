use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use anyhow::{anyhow, Result};

use crate::{solution::Solution, vector::Vec2, vector_map::VectorMap};

pub struct Day18;

impl Solution for Day18 {
    type Answer = String;
    fn day(&self) -> u8 {
        18
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let maze: MemoryMaze = input.parse()?;
        let take = if cfg!(test) { 12 } else { 1024 };
        let map = maze.to_vector_map(take);

        map.find_shortest_path()
            .ok_or(anyhow!("No path found"))
            .map(|steps| steps.to_string())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let maze: MemoryMaze = input.parse()?;
        let initial_take = if cfg!(test) { 0 } else { 1024 };
        let mut map = maze.to_vector_map(initial_take);
        for take in initial_take..maze.bytes.len() {
            map.0.set(&maze.bytes[take], true);
            let steps = map.find_shortest_path();
            if let None = steps {
                return Ok(maze.bytes[take].to_string());
            }
        }
        Err(anyhow!("No blocking byte found"))
    }
}

struct MemoryMaze {
    bytes: Vec<Vec2>,
    size: Vec2,
}

struct MemoryMap(VectorMap<bool>);

impl MemoryMaze {
    fn to_vector_map(&self, take: usize) -> MemoryMap {
        let mut map = VectorMap::new(self.size, false);

        for pos in self.bytes.iter().take(take) {
            map.set(pos, true);
        }

        MemoryMap(map)
    }
}

impl MemoryMap {
    fn find_shortest_path(&self) -> Option<usize> {
        let start = Vec2::new(0, 0);
        let end = self.0.size() - Vec2::new(1, 1);

        let mut visited = HashMap::new();
        let mut queue: BinaryHeap<Step> = BinaryHeap::new();

        queue.push(Step(0, start));

        while let Some(Step(steps, pos)) = queue.pop() {
            if pos == end {
                return Some(steps);
            }

            if let Some(&prev_steps) = visited.get(&pos) {
                if prev_steps <= steps {
                    continue;
                }
            }

            visited.insert(pos, steps);

            for neighbor in pos.get_surrounding_4() {
                if let Some(false) = self.0.get(&neighbor) {
                    queue.push(Step(steps + 1, neighbor));
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step(usize, Vec2);

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl FromStr for MemoryMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_str, bytes_str) = s.split_once("\n\n").unwrap();

        let size = size_str.parse()?;

        let bytes = bytes_str
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        Ok(Self { bytes, size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day18.run_test1(), "22");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day18.run_test2(), "6,1");
    }
}
