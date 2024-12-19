use std::{collections::VecDeque, str::FromStr};

use anyhow::{Ok, Result};
use rayon::prelude::*;

use crate::solution::Solution;

pub struct Day07;

impl Solution for Day07 {
    type Answer = u64;
    fn day(&self) -> u8 {
        7
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let equations: Vec<Equation> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        let possibly_true_results = equations
            .par_iter()
            .filter(|equation| equation.possibly_true::<false>())
            .map(|equation| equation.result)
            .sum();

        Ok(possibly_true_results)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let equations: Vec<Equation> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        let possibly_true_results = equations
            .par_iter()
            .filter(|equation| equation.possibly_true::<true>())
            .map(|equation| equation.result)
            .sum();

        Ok(possibly_true_results)
    }
}

#[derive(Clone, Debug)]
struct Equation {
    result: u64,
    ns: Vec<u64>,
}

impl Equation {
    fn possibly_true<const WITH_CONCAT: bool>(&self) -> bool {
        let mut edge = VecDeque::new();
        edge.push_back(self.clone());
        while let Some(equation) = edge.pop_front() {
            match equation.ns.as_slice() {
                [a] => {
                    if *a == equation.result {
                        return true;
                    }
                }
                [a, b, rest @ ..] => {
                    if equation.result < *a {
                        // a is already too large
                        // no further operations will bring us to the result
                        continue;
                    }

                    let sum = a + b;
                    let product = a * b;

                    let ns_with_sum = [&[sum], rest].concat();
                    let ns_with_product = [&[product], rest].concat();

                    edge.push_back(Equation {
                        ns: ns_with_sum,
                        ..equation
                    });
                    edge.push_back(Equation {
                        ns: ns_with_product,
                        ..equation
                    });

                    if WITH_CONCAT {
                        let log10_b = b.ilog10() + 1;
                        let concat = a * 10_u64.pow(log10_b as u32) + b;
                        let ns_with_concat = [&[concat], rest].concat();
                        edge.push_back(Equation {
                            ns: ns_with_concat,
                            ..equation
                        });
                    }
                }
                _ => panic!("Invalid equation"),
            }
        }
        false
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result_str, ns_str) = s.split_once(": ").expect("Invalid input");
        let result = result_str.parse()?;
        let ns = ns_str
            .split_whitespace()
            .map(|n| Ok(n.parse()?))
            .collect::<Result<Vec<u64>>>()?;
        Ok(Equation { result, ns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let day = Day07;
        assert_eq!(day.run_test1(), 3749);
    }

    #[test]
    fn part2() {
        let day = Day07;
        assert_eq!(day.run_test2(), 11387);
    }

    #[test]
    fn equation_possibly_true() {
        let equation = "190: 10 19".parse::<Equation>().unwrap();
        assert!(equation.possibly_true::<false>());
    }

    #[test]
    fn equation_possible_true2() {
        let equation = "3267: 81 40 27".parse::<Equation>().unwrap();
        assert!(equation.possibly_true::<false>());
    }

    #[test]
    fn equation_not_possible_true() {
        let equation = "83: 17 5".parse::<Equation>().unwrap();
        assert!(!equation.possibly_true::<false>());
    }
}
