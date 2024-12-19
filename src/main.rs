mod days;

mod solution;
mod vector;
mod vector_map;

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
        9 => Day09.run(),
        10 => Day10.run(),
        11 => Day11.run(),
        12 => Day12.run(),
        13 => Day13.run(),
        14 => Day14.run(),
        15 => Day15.run(),
        16 => Day16.run(),
        17 => Day17.run(),
        18 => Day18.run(),
        19 => Day19.run(),
        n => Err(anyhow!("Day {} not implemented", n)),
    }
}
