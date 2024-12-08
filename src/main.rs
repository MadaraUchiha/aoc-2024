mod days;

mod solution;
mod vector;

mod cli;

use anyhow::{anyhow, Result};
use clap::Parser;
use solution::Solution;

use days::*;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.day {
        1 => Day01.run(),
        2 => Day02.run(),
        3 => Day03.run(),
        4 => Day04.run(),
        5 => Day05.run(),
        6 => Day06.run(),
        7 => Day07.run(),
        8 => Day08.run(),
        n => Err(anyhow!("Day {} not implemented", n)),
    }
}
