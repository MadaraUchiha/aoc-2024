use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::{solution::Solution, vector::Vec2};

pub struct Day13;

impl Solution for Day13 {
    type Answer = i64;
    fn day(&self) -> u8 {
        13
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let machines = input
            .split("\n\n")
            .map(|s| s.parse::<ArcadeMachine>().unwrap());

        let prize_presses = machines.filter_map(|machine| machine.find_prize());

        let total_cost = prize_presses.map(|(a, b)| a * 3.0 + b).sum::<f64>();

        Ok(total_cost as i64)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let mut machines = input
            .split("\n\n")
            .map(|s| s.parse::<ArcadeMachine>().unwrap())
            .collect::<Vec<_>>();

        machines
            .iter_mut()
            .for_each(|machine| machine.adjust_prize_location());

        let prize_presses = machines.iter().filter_map(|machine| machine.find_prize());

        let total_cost = prize_presses.map(|(a, b)| a * 3.0 + b).sum::<f64>();

        Ok(total_cost as i64)
    }
}

struct ArcadeMachine {
    a_button: Vec2,
    b_button: Vec2,
    prize: Vec2,
}

impl ArcadeMachine {
    fn find_prize(&self) -> Option<(f64, f64)> {
        let a = self.a_button;
        let b = self.b_button;
        let c = self.prize;

        let det = (a.x * b.y - a.y * b.x) as f64;
        if det == 0.0 {
            return None;
        }

        let x = (c.x * b.y - c.y * b.x) as f64 / det;
        let y = (a.x * c.y - a.y * c.x) as f64 / det;

        if (x.trunc() - x).abs() > f64::EPSILON || (y.trunc() - y).abs() > f64::EPSILON {
            return None;
        }

        Some((x.trunc(), y.trunc()))
    }

    fn adjust_prize_location(&mut self) {
        self.prize = self.prize + Vec2::new(10000000000000, 10000000000000);
    }
}

impl FromStr for ArcadeMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a_line = lines.next().unwrap();
        let b_line = lines.next().unwrap();
        let prize_line = lines.next().unwrap();

        let a_vec = line_to_vec(a_line);
        let b_vec = line_to_vec(b_line);
        let prize_vec = line_to_vec(prize_line);

        Ok(ArcadeMachine {
            a_button: a_vec,
            b_button: b_vec,
            prize: prize_vec,
        })
    }
}

fn line_to_vec(s: &str) -> Vec2 {
    let parts = s
        .split_whitespace()
        .filter(|part| part.contains("+") || part.contains("="))
        .map(|part| part.trim_matches(&['X', 'Y', '+', '=', ',']))
        .map(|part| part.parse::<i64>().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap();

    parts.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day13.run_test1(), 480);
    }

    #[test]
    fn test_arcade_machine() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let machine: ArcadeMachine = input.parse().unwrap();

        let (x, y) = machine.find_prize().unwrap();
        assert_eq!(x, 80.0);
        assert_eq!(y, 40.0);
        assert_eq!(0.1 + 0.2, 0.3);
    }
}
