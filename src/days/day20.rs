use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    u64,
};

use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::{solution::Solution, vector::Vec2, vector_map::VectorMap};

pub struct Day20;

impl Solution for Day20 {
    type Answer = u64;
    fn day(&self) -> u8 {
        20
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let track = input.parse::<Track>()?;
        let threshold = if cfg!(test) { 10 } else { 100 };
        Ok(track.count_cheats_above_threshold(threshold, 2))
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let track = input.parse::<Track>()?;
        let threshold = if cfg!(test) { 50 } else { 100 };
        Ok(track.count_cheats_above_threshold(threshold, 20))
    }
}

struct Track {
    map: VectorMap<bool>,
    start: Vec2,
}

impl Track {
    fn calculate_costs_map(&self) -> VectorMap<u64> {
        let mut costs = VectorMap::new(self.map.size(), u64::MAX);
        let mut visited = HashSet::new();
        let mut current_cost = 0;
        costs.set(&self.start, 0);

        let mut queue = VecDeque::from([self.start]);

        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            costs.set(&pos, current_cost);
            current_cost += 1;

            for next_pos in pos.get_surrounding_4() {
                if let Some(true) = self.map.get(&next_pos) {
                    queue.push_back(next_pos);
                }
            }
        }

        costs
    }

    fn count_cheats_above_threshold(&self, threshold: u64, max_distance: u64) -> u64 {
        let costs = self.calculate_costs_map();
        let candidates: Vec<_> = costs
            .iter()
            .filter_map(|(pos, &cost)| if cost != u64::MAX { Some(pos) } else { None })
            .collect();

        candidates
            .par_iter()
            .map(|&start| {
                Self::find_cheats(&costs, start, max_distance)
                    .into_iter()
                    .filter(|&value| value >= threshold)
                    .count() as u64
            })
            .sum()
    }

    fn find_cheats(costs: &VectorMap<u64>, start: Vec2, max_distance: u64) -> Vec<u64> {
        let start_cost = *costs.get(&start).unwrap();
        costs
            .iter()
            .filter(|(end, _)| start.manhattan_distance(end) <= max_distance)
            .filter(|(_, &cost)| cost != u64::MAX)
            .filter(|(_, &end_cost)| end_cost > start_cost)
            .map(|(end, &cost)| cost.abs_diff(start_cost) - start.manhattan_distance(&end))
            .collect()
    }
}

impl FromStr for Track {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut map = VectorMap::new((width, height).into(), false);

        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x, y).into();
                match c {
                    '.' | 'E' => {
                        map.set(&pos, true);
                    }
                    '#' => {
                        map.set(&pos, false);
                    }
                    'S' => {
                        start = Some(pos);
                        map.set(&pos, true);
                    }
                    _ => return Err(anyhow!("Invalid character: {} in position {}", c, pos)),
                }
            }
        }
        let start = start.ok_or(anyhow!("No start found"))?;

        Ok(Self { map, start })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day20.run_test1(), 2 + 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day20.run_test2(),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn test_cost_map() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let track = input.parse::<Track>().unwrap();
        let costs = track.calculate_costs_map();
        assert_eq!(costs.get(&track.start).unwrap(), &0);
    }

    #[test]
    fn test_cheat_value() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let track = input.parse::<Track>().unwrap();
        let costs = track.calculate_costs_map();
        assert_eq!(Track::find_cheats(&costs, (1, 3).into(), 2), vec![4]);
        assert_eq!(Track::find_cheats(&costs, (7, 1).into(), 2), vec![12]);
        assert_eq!(Track::find_cheats(&costs, (9, 7).into(), 2), vec![20, 36]);
        assert_eq!(Track::find_cheats(&costs, (8, 7).into(), 2), vec![38]);
    }
}
