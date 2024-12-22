use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::prelude::*;

use crate::solution::Solution;

pub struct Day22;

impl Solution for Day22 {
    type Answer = u64;
    fn day(&self) -> u8 {
        22
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let ns = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Ok(ns
            .into_iter()
            .map(PRNG::new)
            .map(|mut prng| prng.nth(1999).unwrap() as u64)
            .sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let ns = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let secret_numbers = ns
            .into_iter()
            .map(PRNG::new)
            .map(|prng| prng.take(2000).collect::<Vec<_>>());

        let all_first_digits = secret_numbers
            .map(|ns| ns.into_iter().map(|n| n % 10).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let all_differences = all_first_digits
            .iter()
            .map(|digits| {
                digits
                    .iter()
                    .tuple_windows()
                    .map(|(&a, &b)| b as i32 - a as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let differences_index = all_differences
            .par_iter()
            .enumerate()
            .map(|(n_index, diff)| {
                diff.iter()
                    .tuple_windows()
                    .enumerate()
                    .map(|(position, (&a, &b, &c, &d))| {
                        let digits_position = position + 4;
                        ((a, b, c, d), all_first_digits[n_index][digits_position])
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>();

        let all_possible_difference_windows = differences_index
            .iter()
            .flat_map(|index| index.keys())
            .collect::<HashSet<_>>();

        let highest_bananas = all_possible_difference_windows
            .par_iter()
            .map(|diff_pattern| {
                differences_index
                    .iter()
                    .filter_map(|index| index.get(diff_pattern))
                    .map(|&n| n as u64)
                    .sum()
            })
            .max();

        highest_bananas.ok_or_else(|| anyhow!("No solution found"))
    }
}

struct PRNG {
    current: u32,
}

impl PRNG {
    fn new(n: u32) -> Self {
        Self { current: n }
    }

    fn next_secret_number(n: u32) -> u32 {
        let shifted = n << 6;
        let n = Self::mix(n, shifted);
        let n = Self::prune(n);

        let shifted = n >> 5;
        let n = Self::mix(n, shifted);
        let n = Self::prune(n);

        let shifted = n << 11;
        let n = Self::mix(n, shifted);
        let n = Self::prune(n);

        n
    }

    #[inline]
    fn mix(n: u32, shifted: u32) -> u32 {
        n ^ shifted
    }

    #[inline]
    fn prune(n: u32) -> u32 {
        n % 16777216
    }
}

impl Iterator for PRNG {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = Self::next_secret_number(self.current);
        self.current = n;
        Some(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let part1_sample = r#"1
10
100
2024"#;
        assert_eq!(Day22::part1(part1_sample)?, 37327623);
        Ok(())
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day22.run_test2(), 23);
    }

    #[test]
    fn test_prng() {
        let mut prng = PRNG::new(123);
        let next_10 = prng.by_ref().take(10).collect::<Vec<_>>();
        assert_eq!(
            next_10,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }
}
