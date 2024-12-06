use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn day(&self) -> u8 {
        5
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let manual = input.parse::<ManualInstructions>()?;
        let mut result = 0;
        for update in &manual.updates {
            if manual.page_matches_rules(update) {
                let middle_page_number = ManualInstructions::update_middle_page(update);
                result += middle_page_number as i64;
            }
        }
        Ok(result)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let manual = input.parse::<ManualInstructions>()?;
        let mut result = 0;
        for update in &manual.updates {
            if !manual.page_matches_rules(update) {
                let new_ordering = manual.find_correct_ordering(update);
                let middle_page_number = ManualInstructions::update_middle_page(&new_ordering);
                result += middle_page_number as i64;
            }
        }

        Ok(result)
    }
}

#[derive(Debug)]
struct ManualInstructions {
    rules: Vec<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

impl FromStr for ManualInstructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule_str, updates_str) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Failed to split input string"))?;

        let rules = rule_str
            .lines()
            .map(|line| -> Result<(u8, u8)> {
                let (before, after) = line
                    .split_once("|")
                    .ok_or(anyhow!("Failed to split rule"))?;

                Ok((before.parse()?, after.parse()?))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let updates: Vec<Vec<u8>> = updates_str
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|n| -> Result<u8> { Ok(n.parse::<u8>()?) })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<_>>()?;

        Ok(ManualInstructions { rules, updates })
    }
}

impl ManualInstructions {
    fn page_matches_rules(&self, update: &[u8]) -> bool {
        for (before, after) in self.rules_matching_update(update) {
            let first_index = update.iter().position(|&n| n == *before);
            let second_index = update.iter().position(|&n| n == *after);
            match (first_index, second_index) {
                (Some(first), Some(second)) => {
                    if first > second {
                        return false;
                    }
                }
                _ => continue,
            }
        }

        true
    }

    fn rules_matching_update<'a>(
        &'a self,
        update: &'a [u8],
    ) -> impl Iterator<Item = &(u8, u8)> + 'a {
        self.rules
            .iter()
            .filter(move |(before, after)| update.contains(before) && update.contains(after))
    }

    fn find_correct_ordering(&self, update: &[u8]) -> Vec<u8> {
        let mut new_update = vec![0; update.len()];
        let relevant_rules = self.rules_matching_update(update).collect_vec();
        for page in update {
            let new_index_for_page = relevant_rules
                .iter()
                .filter(|(_, after)| *after == *page)
                .count();
            new_update[new_index_for_page] = *page;
        }

        new_update
    }

    fn update_middle_page(update: &[u8]) -> u8 {
        let len = update.len();
        let middle_index = len / 2;

        update[middle_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Day05.run_test1(), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Day05.run_test2(), 123);
    }
}
