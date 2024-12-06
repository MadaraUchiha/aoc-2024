use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

use crate::{
    solution::Solution,
    vector::{Vec2, UP},
};

pub struct Day06;

impl Solution for Day06 {
    fn day(&self) -> u8 {
        6
    }

    fn part1(input: &str) -> Result<i64> {
        let mut lab = input.parse::<Lab>()?;
        Ok(lab.run() as i64)
    }

    fn part2(input: &str) -> Result<i64> {
        let lab = input.parse::<Lab>()?;
        Ok(lab.find_loop_inducing_positions() as i64)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lab {
    map: HashSet<Vec2>,
    size: Vec2,
    guard_position: Vec2,
    guard_direction: Vec2,
}

impl Lab {
    fn take_step(&mut self) {
        let next_position = self.guard_position + self.guard_direction;
        if self.map.contains(&next_position) {
            self.guard_direction = self.guard_direction.rotate_clockwise();
        } else {
            self.guard_position = next_position;
        }
    }

    fn run(&mut self) -> usize {
        let mut visited = HashSet::new();
        loop {
            let within_bounds = self.guard_position.x >= 0
                && self.guard_position.x < self.size.x
                && self.guard_position.y >= 0
                && self.guard_position.y < self.size.y;

            if !within_bounds {
                break;
            }

            visited.insert(self.guard_position);
            self.take_step();
        }

        visited.len()
    }

    fn will_loop(&mut self) -> bool {
        let mut visited = HashSet::new();
        loop {
            let within_bounds = self.guard_position.x >= 0
                && self.guard_position.x < self.size.x
                && self.guard_position.y >= 0
                && self.guard_position.y < self.size.y;

            if !within_bounds {
                return false;
            }

            if visited.contains(&(self.guard_position, self.guard_direction)) {
                return true;
            }

            visited.insert((self.guard_position, self.guard_direction));
            self.take_step();
        }
    }

    fn find_loop_inducing_positions(&self) -> usize {
        let mut count = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let position = Vec2::new(x, y);
                if self.map.contains(&position) {
                    continue;
                }

                let mut cloned_lab = self.clone();
                cloned_lab.map.insert(position);
                if cloned_lab.will_loop() {
                    count += 1;
                }
            }
        }

        count
    }
}

impl FromStr for Lab {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashSet::new();
        let mut guard_position = (0, 0).into();

        let y_size = s.lines().count();
        let x_size = s.lines().next().unwrap().chars().count();
        let size = (x_size, y_size).into();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = (x, y).into();
                match c {
                    '#' => {
                        map.insert(position);
                    }
                    '^' => {
                        guard_position = position;
                    }
                    _ => {}
                }
            }
        }
        Ok(Lab {
            map,
            guard_position,
            guard_direction: UP,
            size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day06.run_test1(), 41);
    }

    #[test]
    fn part2() {
        assert_eq!(Day06.run_test2(), 6);
    }
}
