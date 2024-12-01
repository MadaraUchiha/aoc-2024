use std::{fs, time::Instant};

use anyhow::Result;

pub trait Solution {
    fn day(&self) -> u8;
    fn part1(input: &str) -> Result<i64>;
    fn part2(input: &str) -> Result<i64>;

    fn solve(input: &str) -> Result<()> {
        let start = Instant::now();
        let part1 = Self::part1(input)?;
        println!("Part 1 solution: {}, took: {:?}", part1, start.elapsed());

        let start = Instant::now();
        let part2 = Self::part2(input)?;
        println!("Part 2 solution: {}, took: {:?}", part2, start.elapsed());

        Ok(())
    }

    fn run(&self) -> Result<()> {
        let day = self.day();
        let path = format!("./src/input/day{:02}.txt", day);
        let start = Instant::now();
        let input = fs::read_to_string(path)?;
        println!(
            "Reading input took: {:?}, read {} bytes",
            start.elapsed(),
            input.len()
        );
        Self::solve(&input)
    }

    #[cfg(test)]
    fn run_test1(&self) -> i64 {
        let day = self.day();
        let path = format!("./src/input/sample{:02}.txt", day);
        let input = fs::read_to_string(path).unwrap();
        Self::part1(&input).expect("Part 1 failed")
    }

    #[cfg(test)]
    fn run_test2(&self) -> i64 {
        let day = self.day();
        let path = format!("./src/input/sample{:02}.txt", day);
        let input = fs::read_to_string(path).unwrap();
        Self::part2(&input).expect("Part 2 failed")
    }
}
