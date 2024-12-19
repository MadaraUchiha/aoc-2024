use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

use crate::solution::Solution;

pub struct Day17;

impl Solution for Day17 {
    type Answer = i64;
    fn day(&self) -> u8 {
        17
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let mut computer = input.parse::<ThreeBitComputer>()?;
        computer.run_assmebled_program();
        println!(
            "{}",
            computer
                .output
                .iter()
                .map(|&b| b.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        Ok(0)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let computer = input.parse::<ThreeBitComputer>()?;
        let min_a_reg = computer.find_minimal_a_reg();

        Ok(min_a_reg as i64)
    }
}

#[derive(Clone)]
struct ThreeBitComputer {
    a_reg: u64,

    output: Vec<u8>,
    program: Vec<u8>,
}

impl ThreeBitComputer {
    fn run_assmebled_program(&mut self) {
        loop {
            if self.a_reg == 0 {
                break;
            }
            let out = Self::calc_step(self.a_reg);
            self.output.push(out as u8);
            self.a_reg >>= 3;
        }
    }

    fn find_minimal_a_reg(&self) -> u64 {
        let mut candidates = HashSet::new();
        candidates.insert(0);
        for num in self.program.iter().rev() {
            let mut new_candidates = HashSet::new();
            for candidate in candidates {
                for i in 0..8 {
                    let new = (candidate << 3) + i;
                    if Self::calc_step(new) == *num as u64 {
                        new_candidates.insert(new);
                    }
                }
            }
            candidates = new_candidates;
        }
        *candidates.iter().min().unwrap()
    }

    fn calc_step(a: u64) -> u64 {
        let b = a % 8; // 2,4
        let b = b ^ 7; // 1,7
        let c = a >> b; // 7,5
        let b = b ^ c; // 4,0
        let b = b ^ 7; // 1,7
        b % 8
    }
}

impl FromStr for ThreeBitComputer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a_reg = 0;
        let mut program = Vec::new();

        for line in s.lines().filter(|l| !l.is_empty()) {
            match line.split_once(": ") {
                Some(("Register A", value)) => a_reg = value.parse()?,
                Some(("Register B", _)) => {}
                Some(("Register C", _)) => {}
                Some(("Program", value)) => {
                    program = value
                        .split(",")
                        .map(|v| v.parse())
                        .collect::<Result<_, _>>()?;
                }
                v => return Err(anyhow!("Invalid input: {:?}", v)),
            }
        }

        Ok(Self {
            a_reg,
            program,
            output: Vec::new(),
        })
    }
}
