use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use anyhow::anyhow;

use crate::{
    solution::Solution,
    vector::{Vec2, RIGHT},
    vector_map::VectorMap,
};

pub struct Day15;

impl Solution for Day15 {
    fn day(&self) -> u8 {
        15
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let mut warehouse = input.parse::<Warehouse>()?;
        warehouse.run_instructions();
        Ok(warehouse.gps_score())
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let mut warehouse = input.parse::<Warehouse>()?;
        warehouse = warehouse.widen();
        println!("{}", warehouse);
        // warehouse.run_instructions();
        // Ok(warehouse.gps_score())
        Ok(0)
    }
}

struct Warehouse {
    map: VectorMap<char>,
    robot_position: Vec2,
    instructions: Vec<Vec2>,
}

impl Warehouse {
    fn move_robot(&mut self, direction: Vec2) {
        let new_position = self.robot_position + direction;
        match self.map.get(&new_position) {
            Some('.') => {
                self.robot_position = new_position;
            }
            Some('O') => {
                let mut box_position = new_position;
                while let Some(&tile) = self.map.get(&box_position) {
                    if tile == '#' {
                        break;
                    }
                    if tile == 'O' {
                        box_position = box_position + direction;
                        continue;
                    }
                    self.map.set(&box_position, 'O');
                    self.map.set(&new_position, '.');
                    self.robot_position = new_position;
                    break;
                }
            }
            Some('#') => {}
            _ => panic!("Invalid position"),
        }
        // println!("\n{}", self);
    }

    fn run_instructions(&mut self) {
        let instructions = self.instructions.clone();
        for &direction in &instructions {
            self.move_robot(direction);
        }
    }

    fn gps_score(&self) -> i64 {
        self.map
            .iter()
            .filter(|(_, &c)| c == 'O')
            .map(|(pos, _)| pos.x + pos.y * 100)
            .sum::<i64>()
    }

    fn widen(&self) -> Self {
        let original_size = self.map.size();
        let original_robot_position = self.robot_position;
        let mut new_map = VectorMap::new(Vec2::new(original_size.x * 2, original_size.y), '.');
        let new_robot_position =
            Vec2::new(original_robot_position.x * 2, original_robot_position.y);
        for (pos, &c) in self.map.iter() {
            let t_pos = Vec2::new(pos.x * 2, pos.y);
            match c {
                '.' => {
                    new_map.set(&t_pos, '.');
                    new_map.set(&(t_pos + RIGHT), '.');
                }
                'O' => {
                    new_map.set(&t_pos, '[');
                    new_map.set(&(t_pos + RIGHT), ']');
                }
                '#' => {
                    new_map.set(&t_pos, '#');
                    new_map.set(&(t_pos + RIGHT), '#');
                }
                _ => {}
            }
        }
        Warehouse {
            map: new_map,
            robot_position: new_robot_position,
            instructions: self.instructions.clone(),
        }
    }
}

impl FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_str, instructions_str) = s.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;

        let mut map = map_str.parse::<VectorMap<char>>()?;
        let robot_position = map
            .iter()
            .find(|(_, &c)| c == '@')
            .map(|(pos, _)| pos)
            .unwrap();
        map.set(&robot_position, '.');

        let instructions = instructions_str
            .chars()
            .filter(|c| ['^', 'v', '<', '>'].contains(c))
            .map(|c| match c {
                '^' => Vec2::new(0, -1),
                'v' => Vec2::new(0, 1),
                '<' => Vec2::new(-1, 0),
                '>' => Vec2::new(1, 0),
                _ => panic!("Invalid instruction"),
            })
            .collect();

        Ok(Warehouse {
            map,
            robot_position,
            instructions,
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut map = self.map.clone();
        map.set(&self.robot_position, '@');
        write!(f, "{}", map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day15.run_test1(), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day15.run_test2(), 0);
    }

    #[test]
    fn test_small_example() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let mut warehouse = input.parse::<Warehouse>().unwrap();
        warehouse.run_instructions();
        println!("{}", warehouse);
        assert_eq!(warehouse.gps_score(), 24);
    }
}
