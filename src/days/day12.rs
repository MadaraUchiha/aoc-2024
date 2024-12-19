use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;

use crate::{
    solution::Solution,
    vector::{Vec2, DOWN, LEFT, RIGHT, UP},
    vector_map::VectorMap,
};

pub struct Day12;

impl Solution for Day12 {
    type Answer = usize;
    fn day(&self) -> u8 {
        12
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let garden = input.parse::<Garden>()?;
        let regions = garden.find_all_regions();

        Ok(regions.iter().map(|region| region.price()).sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let garden = input.parse::<Garden>()?;
        let regions = garden.find_all_regions();

        let result = regions
            .iter()
            .map(|region| region.area() * garden.count_corners(region))
            .sum();
        Ok(result)
    }
}

struct Garden {
    map: VectorMap<char>,
}

impl Garden {
    fn find_all_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited: HashSet<Vec2> = HashSet::new();

        for (pos, _) in self.map.iter() {
            if visited.contains(&pos) {
                continue;
            }

            let region = self.find_region_around(pos);
            visited.extend(&region.1);
            regions.push(region);
        }

        regions
    }

    fn find_region_around(&self, pos: Vec2) -> Region {
        let mut region = HashSet::new();
        let mut to_visit = VecDeque::from([pos]);

        let plant = *self.map.get(&pos).unwrap();

        while let Some(curr_pos) = to_visit.pop_front() {
            if region.contains(&curr_pos) {
                continue;
            }

            if self.map.get(&curr_pos) != Some(&plant) {
                continue;
            }

            region.insert(curr_pos);

            for neighbor in curr_pos.get_surrounding_4() {
                to_visit.push_back(neighbor);
            }
        }

        Region(plant, region)
    }

    fn count_corners(&self, region: &Region) -> usize {
        let mut corners = 0;
        let mut triple_corners = 0;

        let corners_around = |&pos: &Vec2| {
            [
                (pos + UP + LEFT, pos + UP, pos + LEFT, pos),
                (pos + UP, pos + UP + RIGHT, pos, pos + RIGHT),
                (pos + LEFT, pos, pos + DOWN + LEFT, pos + DOWN),
                (pos, pos + RIGHT, pos + DOWN, pos + DOWN + RIGHT),
            ]
        };

        let corner_patterns = [
            (false, false, false, true),
            (false, false, true, false),
            (false, true, false, false),
            (true, false, false, false),
            (false, true, true, false),
            (true, false, false, true),
        ];

        let overcounted_patterns = [
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, true),
            (false, true, true, true),
        ];

        for pos in &region.1 {
            for (up_left_pos, up_right_pos, down_left_pos, down_right_pos) in &corners_around(&pos)
            {
                let pattern = (
                    self.map
                        .get(up_left_pos)
                        .map(|&c| c == region.0)
                        .unwrap_or(false),
                    self.map
                        .get(up_right_pos)
                        .map(|&c| c == region.0)
                        .unwrap_or(false),
                    self.map
                        .get(down_left_pos)
                        .map(|&c| c == region.0)
                        .unwrap_or(false),
                    self.map
                        .get(down_right_pos)
                        .map(|&c| c == region.0)
                        .unwrap_or(false),
                );

                if corner_patterns.contains(&pattern) {
                    corners += 1;
                }

                if overcounted_patterns.contains(&pattern) {
                    triple_corners += 1;
                }
            }
        }

        corners + triple_corners / 3
    }
}

impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.parse::<VectorMap<char>>()?;
        Ok(Garden { map })
    }
}

#[derive(Debug)]
struct Region(char, HashSet<Vec2>);

impl Region {
    fn area(&self) -> usize {
        self.1.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for pos in &self.1 {
            for neighbor in pos.get_surrounding_4() {
                if !self.1.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Day12.run_test1(), 1930)
    }

    #[test]
    fn part2_example() {
        assert_eq!(Day12.run_test2(), 1206)
    }
}
