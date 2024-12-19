use std::collections::HashMap;

use anyhow::Result;

use crate::solution::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Answer = i64;
    fn day(&self) -> u8 {
        1
    }

    fn part1(input: &str) -> Result<i64> {
        let (mut left, mut right) = Day01::parse_input(input);

        left.sort();
        right.sort();

        let sorted_pairs = left.iter().zip(right.iter());

        Ok(sorted_pairs
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>()
            .into())
    }

    fn part2(input: &str) -> Result<i64> {
        let (right, left) = Day01::parse_input(input);

        let mut frequency: HashMap<u32, u32> = HashMap::new();
        for n in left {
            *frequency.entry(n).or_default() += 1;
        }

        let mut sum = 0;
        for n in right {
            if let Some(&count) = frequency.get(&n) {
                sum += n * count;
            }
        }

        Ok(sum.into())
    }
}

impl Day01 {
    fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
        input
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(a, b)| {
                (
                    a.trim().parse::<u32>().unwrap(),
                    b.trim().parse::<u32>().unwrap(),
                )
            })
            .unzip()
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
