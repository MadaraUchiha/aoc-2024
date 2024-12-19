use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

use crate::{solution::Solution, vector::Vec2};

pub struct Day08;

impl Solution for Day08 {
    type Answer = i64;
    fn day(&self) -> u8 {
        8
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let antenna_map = input.parse::<AntennaMap>()?;
        let total_antinodes = antenna_map
            .antenna_index
            .keys()
            .map(|c| antenna_map.get_antinodes(*c))
            .reduce(|a, b| a.union(&b).copied().collect())
            .unwrap()
            .len();

        Ok(total_antinodes as i64)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let antenna_map = input.parse::<AntennaMap>()?;
        let total_antinodes = antenna_map
            .antenna_index
            .keys()
            .map(|c| antenna_map.get_antinodes_projections(*c))
            .reduce(|a, b| a.union(&b).copied().collect())
            .unwrap()
            .len();

        Ok(total_antinodes as i64)
    }
}

struct AntennaMap {
    antenna_index: HashMap<char, Vec<Vec2>>,
    size: Vec2,
}

impl AntennaMap {
    fn get_antinodes(&self, c: char) -> HashSet<Vec2> {
        let mut result = HashSet::new();
        let pairs = self
            .antenna_index
            .get(&c)
            .unwrap()
            .iter()
            .tuple_combinations();

        for (a, b) in pairs {
            let diff = *b - *a;
            let antinode1 = *b + diff;
            let antinode2 = *a - diff;

            if antinode1.contained_in(Vec2::new(0, 0), self.size) {
                result.insert(antinode1);
            }
            if antinode2.contained_in(Vec2::new(0, 0), self.size) {
                result.insert(antinode2);
            }
        }

        result
    }

    fn get_antinodes_projections(&self, c: char) -> HashSet<Vec2> {
        let mut result = HashSet::new();
        let pairs = self
            .antenna_index
            .get(&c)
            .unwrap()
            .iter()
            .tuple_combinations();

        for (a, b) in pairs {
            let diff = *b - *a;
            let mut antinode1 = *b + diff;
            let mut antinode2 = *b - diff;

            result.insert(*b);

            while antinode1.contained_in(Vec2::new(0, 0), self.size) {
                result.insert(antinode1);
                antinode1 = antinode1 + diff;
            }
            while antinode2.contained_in(Vec2::new(0, 0), self.size) {
                result.insert(antinode2);
                antinode2 = antinode2 - diff;
            }
        }

        result
    }
}

impl FromStr for AntennaMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index = HashMap::new();

        let size = Vec2::new(
            s.lines().next().unwrap().len() as i64,
            s.lines().count() as i64,
        );

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => continue,
                    _ => {
                        let entry = index.entry(c).or_insert_with(Vec::new);
                        entry.push((x, y).into());
                    }
                }
            }
        }

        Ok(Self {
            size,
            antenna_index: index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let day = Day08;
        assert_eq!(day.run_test1(), 14);
    }

    #[test]
    fn part2() {
        let day = Day08;
        assert_eq!(day.run_test2(), 34);
    }

    #[test]
    fn antinode_finder() {
        let antenna_map = AntennaMap {
            antenna_index: HashMap::from([('A', vec![Vec2::new(3, 2), Vec2::new(6, 1)])]),
            size: Vec2::new(10, 10),
        };

        let antinodes = antenna_map.get_antinodes('A');
        assert_eq!(antinodes.len(), 2);
        assert!(antinodes.contains(&Vec2::new(0, 3)));
        assert!(antinodes.contains(&Vec2::new(9, 0)));
    }
}
