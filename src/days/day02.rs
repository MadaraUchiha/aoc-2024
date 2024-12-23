use std::str::FromStr;

use anyhow::{Error, Ok, Result};
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day02;

impl Solution for Day02 {
    type Answer = usize;
    fn day(&self) -> u8 {
        2
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let parsed_input = Self::parse_input(input)?;
        let safe_sequences = parsed_input.iter().filter(|seq| seq.is_safe(None)).count();
        Ok(safe_sequences)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let parsed_input = Self::parse_input(input)?;
        let safe_sequences = parsed_input
            .into_iter()
            .filter(|seq| seq.is_safe_with_tolerance())
            .count();

        Ok(safe_sequences)
    }
}

impl Day02 {
    fn parse_input(input: &str) -> Result<Vec<LevelSequence>> {
        input.lines().map(|line| line.parse()).collect()
    }
}

#[derive(Debug)]
struct LevelSequence(Vec<u8>);

impl FromStr for LevelSequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LevelSequence(
            s.split_ascii_whitespace()
                .map(|s| s.parse().map_err(Error::from))
                .collect::<Result<_>>()?,
        ))
    }
}

impl LevelSequence {
    fn is_safe(&self, skip_index: Option<usize>) -> bool {
        // sequence is safe if it's strictly increasing or strictly decreasing, and by steps of at least 1 and at most 3
        let mut is_increasing: Option<bool> = None;

        for (a, b) in self
            .0
            .iter()
            .enumerate()
            .filter(|(i, _)| skip_index.is_none_or(|s| s != *i))
            .map(|(_, n)| n)
            .tuple_windows()
        {
            if a == b {
                return false;
            }
            match is_increasing {
                None => {
                    is_increasing = Some(a < b);
                }
                Some(true) => {
                    if a > b {
                        return false;
                    }
                }
                Some(false) => {
                    if a < b {
                        return false;
                    }
                }
            }
            let abs_diff = a.abs_diff(*b);
            if !(1..=3).contains(&abs_diff) {
                return false;
            }
        }

        true
    }

    fn is_safe_with_tolerance(&self) -> bool {
        // without elimination
        if self.is_safe(None) {
            return true;
        }

        // with elimination
        for i in 0..self.0.len() {
            if self.is_safe(Some(i)) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_sequence_is_safe() {
        let seq = LevelSequence(vec![1, 2, 3, 4]);
        assert!(seq.is_safe(None));

        let seq = LevelSequence(vec![4, 3, 2, 1]);
        assert!(seq.is_safe(None));

        let seq = LevelSequence(vec![1, 2, 4, 5]);
        assert!(seq.is_safe(None));

        let seq = LevelSequence(vec![1, 2, 4, 3]);
        assert!(!seq.is_safe(None));

        let seq = LevelSequence(vec![1, 2, 4, 10]);
        assert!(!seq.is_safe(None));
    }

    #[test]
    fn test_level_sequence_is_safe_with_tolerance() {
        let seq = LevelSequence(vec![1, 2, 3, 4]);
        assert!(seq.is_safe_with_tolerance());

        let seq = LevelSequence(vec![4, 3, 2, 1]);
        assert!(seq.is_safe_with_tolerance());

        let seq = LevelSequence(vec![1, 2, 4, 5]);
        assert!(seq.is_safe_with_tolerance());

        let seq = LevelSequence(vec![1, 2, 4, 3]);
        assert!(!seq.is_safe(None));
        assert!(seq.is_safe_with_tolerance());

        let seq = LevelSequence(vec![1, 2, 4, 10]);
        assert!(!seq.is_safe(None));
        assert!(seq.is_safe_with_tolerance());
    }

    #[test]
    fn test_day02_part1() {
        assert_eq!(Day02.run_test1(), 2);
    }

    #[test]
    fn test_day02_part2() {
        assert_eq!(Day02.run_test2(), 4);
    }
}
