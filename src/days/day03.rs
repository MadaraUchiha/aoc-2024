use anyhow::{anyhow, Result};
use regex::{Captures, Regex};

use crate::solution::Solution;

pub struct Day03;

const MUL_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const FULL_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";

impl Solution for Day03 {
    type Answer = usize;
    fn day(&self) -> u8 {
        3
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let mul_pattern = Regex::new(MUL_PATTERN).unwrap();

        let mut result = 0;
        for instruction in mul_pattern.captures_iter(input).map(Instruction::try_from) {
            match instruction? {
                Instruction::Mul(a, b) => {
                    result += a * b;
                }
                _ => {}
            }
        }
        Ok(result)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let pattern = Regex::new(FULL_PATTERN).unwrap();
        let mut result = 0;
        let mut active = true;
        for instruction in pattern.captures_iter(input).map(Instruction::try_from) {
            match instruction? {
                Instruction::Do => active = true,
                Instruction::Dont => active = false,
                Instruction::Mul(a, b) => result += if active { a * b } else { 0 },
            }
        }
        Ok(result)
    }
}

#[derive(Debug)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

impl TryFrom<Captures<'_>> for Instruction {
    type Error = anyhow::Error;

    fn try_from(cap: Captures) -> Result<Self> {
        let instruction_name = &cap[0][..cap[0].find("(").ok_or(anyhow!("Invalid instruction"))?];
        match instruction_name {
            "mul" => {
                let a = cap[1].parse()?;
                let b = cap[2].parse()?;
                Ok(Instruction::Mul(a, b))
            }
            "do" => Ok(Instruction::Do),
            "don't" => Ok(Instruction::Dont),
            _ => Err(anyhow!("Invalid instruction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day03.run_test1(), 161)
    }

    #[test]
    fn part2() {
        assert_eq!(Day03.run_test2(), 48)
    }
}
