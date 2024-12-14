use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

use crate::{solution::Solution, vector::Vec2};

pub struct Day14;

impl Solution for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let board = input.parse::<Board>()?;
        let board_after_100 = board.calculate_robots_after(100);
        let safety_score = board_after_100.calculate_safety_score();
        Ok(safety_score)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let mut board = input.parse::<Board>()?;
        let mut seconds = 0;
        while !board.robots_all_in_unique_positions() {
            board = board.calculate_robots_after(1);
            seconds += 1;
        }
        Ok(seconds)
    }
}

struct Board {
    size: Vec2,
    robots: Vec<Robot>,
}

impl Board {
    fn calculate_position_after(&self, robot: &Robot, seconds: i32) -> Vec2 {
        (robot.position + robot.velocity * seconds) % self.size
    }

    fn calculate_robots_after(&self, seconds: i32) -> Self {
        let new_robots = self
            .robots
            .iter()
            .map(|robot| Robot {
                position: self.calculate_position_after(robot, seconds),
                velocity: robot.velocity,
            })
            .collect();

        Board {
            size: self.size,
            robots: new_robots,
        }
    }

    fn calculate_safety_score(&self) -> i64 {
        let half_width = self.size.x / 2;
        let half_height = self.size.y / 2;

        let mut quardrants = [0; 4];

        for robot in &self.robots {
            let x = robot.position.x;
            let y = robot.position.y;

            if x < half_width && y < half_height {
                quardrants[0] += 1;
            } else if x > half_width && y < half_height {
                quardrants[1] += 1;
            } else if x < half_width && y > half_height {
                quardrants[2] += 1;
            } else if x > half_width && y > half_height {
                quardrants[3] += 1;
            }
        }

        let quardants_prudct = quardrants.iter().product::<i64>();
        quardants_prudct
    }

    fn robots_all_in_unique_positions(&self) -> bool {
        let mut positions = HashSet::new();
        for robot in &self.robots {
            if !positions.insert(robot.position) {
                return false;
            }
        }
        true
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_str, robots_str) = s
            .split_once("\n\n")
            .ok_or(anyhow!("Invalid board format"))?;

        let (size_x, size_y) = size_str
            .split_once("x")
            .ok_or(anyhow!("Invalid board size format"))?;
        let size = Vec2::new(size_x.parse()?, size_y.parse()?);

        let robots = robots_str
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Robot>, _>>()?;

        Ok(Board { size, robots })
    }
}

struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_str, v_str) = s.split_once(" ").ok_or(anyhow!("Invalid robot format"))?;

        let p_comma = p_str.trim_matches(&['p', '=']);
        let (p_x, p_y) = p_comma
            .split_once(",")
            .ok_or(anyhow!("Invalid robot position format"))?;
        let p_vec = Vec2::new(p_x.parse()?, p_y.parse()?);

        let v_comma = v_str.trim_matches(&['v', '=']);
        let (v_x, v_y) = v_comma
            .split_once(",")
            .ok_or(anyhow!("Invalid robot velocity format"))?;
        let v_vec = Vec2::new(v_x.parse()?, v_y.parse()?);

        Ok(Robot {
            position: p_vec,
            velocity: v_vec,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day14.run_test1(), 12);
    }
}
