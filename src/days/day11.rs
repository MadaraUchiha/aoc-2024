use std::{collections::HashMap, str::FromStr};

use anyhow::Result;

use crate::solution::Solution;

pub struct Day11;

impl Solution for Day11 {
    type Answer = u64;
    fn day(&self) -> u8 {
        11
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let arrangement = input.parse::<StoneArrangement>()?;
        let final_stones = arrangement.blinks(25);

        Ok(final_stones)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let arrangement = input.parse::<StoneArrangement>()?;
        let final_stones = arrangement.blinks(75);

        Ok(final_stones)
    }
}

#[derive(Clone, Debug)]
struct StoneArrangement {
    stones: HashMap<u64, u64>,
}

impl StoneArrangement {
    fn blinks(&self, n: u64) -> u64 {
        let mut current_stones = self.stones.clone();

        for _ in 0..n {
            let mut next_stones = HashMap::new();
            for (stone, count) in current_stones.iter() {
                let new_stones = stone_step(stone);
                for new_stone in new_stones {
                    let new_count = next_stones.get(&new_stone).unwrap_or(&0) + count;
                    next_stones.insert(new_stone, new_count);
                }
            }
            current_stones = next_stones;
        }

        current_stones.values().sum()
    }
}

fn stone_step(stone: &u64) -> Vec<u64> {
    if *stone == 0 {
        return vec![1];
    }
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let half = digits / 2;
        let upper = stone / 10_u64.pow(half);
        let lower = stone % 10_u64.pow(half);
        return vec![upper, lower];
    }

    vec![stone * 2024]
}

impl FromStr for StoneArrangement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_whitespace()
            .map(|s| s.parse().map(|i| (i, 1)))
            .collect::<Result<_, _>>()?;
        Ok(Self { stones })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day11;
        assert_eq!(day.run_test1(), 55312);
    }
}
