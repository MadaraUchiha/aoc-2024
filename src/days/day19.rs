use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::solution::Solution;

pub struct Day19;

impl Solution for Day19 {
    type Answer = usize;
    fn day(&self) -> u8 {
        19
    }

    fn part1(input: &str) -> Result<usize> {
        let rack = TowelRack::try_from(input)?;
        Ok(rack.count_reachable_patterns())
    }

    fn part2(input: &str) -> Result<usize> {
        let rack = TowelRack::try_from(input)?;
        Ok(rack.count_all_possible_patterns())
    }
}

struct TowelRack<'a> {
    available_sub_patterns: Vec<&'a str>,
    towels: Vec<&'a str>,
}

impl<'a> TowelRack<'a> {
    fn count_possible_towel_patterns(&self, pattern: &str) -> usize {
        fn count_recursive(
            available_sub_patterns: &[&str],
            pattern: &str,
            start: usize,
            memo: &mut HashMap<usize, usize>,
        ) -> usize {
            if let Some(&count) = memo.get(&start) {
                return count;
            }

            let mut count = 0;
            for sub_pattern in available_sub_patterns {
                if pattern[start..].starts_with(sub_pattern) {
                    let next_start = start + sub_pattern.len();
                    if next_start == pattern.len() {
                        count += 1;
                    } else {
                        // Where art thou TCO when I need thee?
                        count += count_recursive(available_sub_patterns, pattern, next_start, memo);
                    }
                }
            }

            memo.insert(start, count);
            count
        }

        let mut memo = HashMap::new();
        count_recursive(&self.available_sub_patterns, pattern, 0, &mut memo)
    }

    fn count_reachable_patterns(&self) -> usize {
        self.towels
            .par_iter()
            .filter(|pattern| self.count_possible_towel_patterns(pattern) != 0)
            .count()
    }

    fn count_all_possible_patterns(&self) -> usize {
        self.towels
            .par_iter()
            .map(|pattern| self.count_possible_towel_patterns(pattern))
            .sum()
    }
}

impl<'a> TryFrom<&'a str> for TowelRack<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self> {
        let (towels_str, patterns_str) = input
            .split_once("\n\n")
            .ok_or(anyhow!("Failed to split input string"))?;

        let towels = towels_str.split(", ").collect();
        let patterns = patterns_str.lines().collect();

        Ok(TowelRack {
            available_sub_patterns: towels,
            towels: patterns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day19.run_test1(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day19.run_test2(), 16);
    }
}
