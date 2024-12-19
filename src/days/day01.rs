use std::collections::HashMap;

use anyhow::Result;
use itertools::process_results;

use crate::solution::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Answer = usize;
    fn day(&self) -> u8 {
        1
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let (mut left, mut right) = Day01::parse_input(input)?;

        left.sort();
        right.sort();

        let sorted_pairs = left.iter().zip(right.iter());

        Ok(sorted_pairs.map(|(a, b)| a.abs_diff(*b)).sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let (right, left) = Day01::parse_input(input)?;

        let mut frequency = HashMap::new();
        for n in left {
            *frequency.entry(n).or_insert(0) += 1;
        }

        let mut sum = 0;
        for n in right {
            if let Some(&count) = frequency.get(&n) {
                sum += n * count;
            }
        }

        Ok(sum)
    }
}

impl Day01 {
    fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
        process_results(
            input
                .lines()
                .map(|line| line.split_once(" ").unwrap())
                .map(|(a, b)| Ok((a.trim().parse::<usize>()?, b.trim().parse::<usize>()?))),
            |pairs| pairs.unzip(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sample1 = Day01.run_test1();
        assert_eq!(sample1, 11);
    }

    #[test]
    fn part2() {
        let sample2 = Day01.run_test2();
        assert_eq!(sample2, 31);
    }
}
