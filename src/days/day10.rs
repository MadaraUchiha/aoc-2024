use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;

use crate::{solution::Solution, vector::Vec2};

pub struct Day10;

impl Solution for Day10 {
    type Answer = u64;
    fn day(&self) -> u8 {
        10
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let map = input.parse::<TopologicalMap>()?;
        Ok(map
            .zeros
            .iter()
            .map(|&zero| map.count_trails::<true>(zero))
            .sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let map = input.parse::<TopologicalMap>()?;
        Ok(map
            .zeros
            .iter()
            .map(|&zero| map.count_trails::<false>(zero))
            .sum())
    }
}

struct TopologicalMap {
    map: Vec<Vec<u8>>,
    size: Vec2,
    zeros: Vec<Vec2>,
}

impl TopologicalMap {
    fn count_trails<const DISTINCT: bool>(&self, start: Vec2) -> u64 {
        let mut trails = HashSet::new();
        let mut total_trails = 0;
        let mut edge = VecDeque::from([start]);
        while let Some(pos) = edge.pop_front() {
            let value = self.get(pos);
            if value == 9 {
                if DISTINCT && trails.insert(pos) {
                    total_trails += 1;
                } else if !DISTINCT {
                    total_trails += 1;
                }
                continue;
            }
            let candidates = pos.get_surrounding_4();

            candidates
                .iter()
                .filter(|&c| c.contained_in(Vec2::new(0, 0), self.size))
                .filter(|&c| self.get(*c) == value + 1)
                .for_each(|c| {
                    edge.push_back(*c);
                });
        }

        total_trails
    }

    fn get(&self, pos: Vec2) -> u8 {
        self.map[pos.y as usize][pos.x as usize]
    }
}

impl FromStr for TopologicalMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .unwrap();

        let size = (map[0].len(), map.len()).into();
        let zeros = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &c)| match c {
                        0 => Some((x, y).into()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(TopologicalMap { map, size, zeros })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day10;
        assert_eq!(day.run_test1(), 36);
    }

    #[test]
    fn test_part2() {
        let day = Day10;
        assert_eq!(day.run_test2(), 81);
    }
}
