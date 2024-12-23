use std::{
    collections::{HashMap, VecDeque},
    iter,
};

use itertools::Itertools;

use crate::{solution::Solution, vector::Vec2};

pub struct Day21;

impl Solution for Day21 {
    type Answer = usize;
    fn day(&self) -> u8 {
        21
    }

    fn part1(input: &str) -> anyhow::Result<Self::Answer> {
        let np = Keypad::numeric();
        let dp = Keypad::directional();
        let mut cache = HashMap::new();
        let result = input
            .lines()
            .map(|line| {
                shortest_len(&np, &dp, String::from(line), 0, 2, &mut cache)
                    * line[0..3].parse::<usize>().unwrap()
            })
            .sum();

        Ok(result)
    }

    fn part2(input: &str) -> anyhow::Result<Self::Answer> {
        let np = Keypad::numeric();
        let dp = Keypad::directional();
        let mut cache = HashMap::new();
        let result = input
            .lines()
            .map(|line| {
                shortest_len(&np, &dp, String::from(line), 0, 25, &mut cache)
                    * line[0..3].parse::<usize>().unwrap()
            })
            .sum();

        Ok(result)
    }
}

fn paths(a: Vec2, b: Vec2, gap: Vec2) -> Vec<String> {
    let mut q = VecDeque::from([(a, String::new())]);
    let mut res = vec![];
    while let Some((Vec2 { x, y }, mut path)) = q.pop_front() {
        if Vec2::new(x, y) == b {
            path.push('A');
            res.push(path);
            continue;
        }
        // left
        if b.y < y && !(gap.x == x && gap.y < y && gap.y >= b.y) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('<').take((y - b.y).try_into().unwrap()));
            q.push_back((Vec2::new(x, b.y), new_path));
        }
        // up
        if b.x < x && !(gap.y == y && gap.x < x && gap.x >= b.x) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('^').take((x - b.x).try_into().unwrap()));
            q.push_back((Vec2::new(b.x, y), new_path));
        }
        // down
        if b.x > x && !(gap.y == y && gap.x > x && gap.x <= b.x) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('v').take((b.x - x).try_into().unwrap()));
            q.push_back((Vec2::new(b.x, y), new_path));
        }
        // right
        if b.y > y && !(gap.x == x && gap.y > y && gap.y <= b.y) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('>').take((b.y - y).try_into().unwrap()));
            q.push_back((Vec2::new(x, b.y), new_path));
        }
    }
    res
}

struct Keypad {
    key_map: HashMap<char, Vec2>,
    gap: Vec2,
}

impl Keypad {
    fn numeric() -> Self {
        let key_map = HashMap::from([
            ('7', Vec2::new(0, 0)),
            ('8', Vec2::new(0, 1)),
            ('9', Vec2::new(0, 2)),
            ('4', Vec2::new(1, 0)),
            ('5', Vec2::new(1, 1)),
            ('6', Vec2::new(1, 2)),
            ('1', Vec2::new(2, 0)),
            ('2', Vec2::new(2, 1)),
            ('3', Vec2::new(2, 2)),
            ('0', Vec2::new(3, 1)),
            ('A', Vec2::new(3, 2)),
        ]);
        let gap = Vec2::new(3, 0);
        Keypad { key_map, gap }
    }

    fn directional() -> Self {
        let key_map = HashMap::from([
            ('^', Vec2::new(0, 1)),
            ('A', Vec2::new(0, 2)),
            ('<', Vec2::new(1, 0)),
            ('v', Vec2::new(1, 1)),
            ('>', Vec2::new(1, 2)),
        ]);
        let gap = Vec2::new(0, 0);
        Keypad { key_map, gap }
    }

    fn paths(&self, a: char, b: char) -> Vec<String> {
        paths(self.key_map[&a], self.key_map[&b], self.gap)
    }
}

fn shortest_len(
    np: &Keypad,
    dp: &Keypad,
    code: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, code.clone())) {
        return cached;
    }

    let kp = if depth == 0 { np } else { dp };
    let res = iter::once('A')
        .chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = kp.paths(a, b);
            if depth == max_depth {
                paths.iter().map(String::len).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| shortest_len(np, dp, path, depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();

    cache.insert((depth, code), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day21.run_test1(), 126384)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day21.run_test2(), 0)
    }
}
