mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

mod solution;
mod vector;

mod cli;

use anyhow::{anyhow, Result};
use clap::Parser;
use solution::Solution;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.day {
        1 => day01::Day01.run().unwrap(),
        2 => day02::Day02.run().unwrap(),
        3 => day03::Day03.run().unwrap(),
        4 => day04::Day04.run().unwrap(),
        5 => day05::Day05.run().unwrap(),
        6 => day06::Day06.run().unwrap(),
        7 => day07::Day07.run().unwrap(),
        n => return Err(anyhow!("Day {} not implemented", n)),
    }
    Ok(())
}
