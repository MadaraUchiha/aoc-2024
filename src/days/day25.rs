use std::str::FromStr;

use anyhow::Result;

use crate::solution::Solution;

pub struct Day25;

impl Solution for Day25 {
    type Answer = u64;
    fn day(&self) -> u8 {
        25
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let schematics = Schematics::from_str(input)?;
        Ok(schematics.count_matching_keys())
    }

    fn part2(_: &str) -> Result<Self::Answer> {
        Ok(1337)
    }
}

struct Schematics {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl Schematics {
    fn count_matching_keys(&self) -> u64 {
        let mut count = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if lock.key_fits(key) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl FromStr for Schematics {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        for block in s.split("\n\n") {
            if block.starts_with(".....") {
                keys.push(Key::from_str(block)?);
            } else if block.starts_with("#####") {
                locks.push(Lock::from_str(block)?);
            }
        }

        Ok(Schematics { keys, locks })
    }
}

#[derive(Debug, PartialEq)]
struct Key(u8, u8, u8, u8, u8);

impl Key {
    fn new(tumbles: (u8, u8, u8, u8, u8)) -> Self {
        Self(tumbles.0, tumbles.1, tumbles.2, tumbles.3, tumbles.4)
    }
}

#[derive(Debug, PartialEq)]
struct Lock(u8, u8, u8, u8, u8);

impl Lock {
    fn new(tumbles: (u8, u8, u8, u8, u8)) -> Self {
        Self(tumbles.0, tumbles.1, tumbles.2, tumbles.3, tumbles.4)
    }
}

impl Lock {
    fn key_fits(&self, key: &Key) -> bool {
        self.0 + key.0 <= 5
            && self.1 + key.1 <= 5
            && self.2 + key.2 <= 5
            && self.3 + key.3 <= 5
            && self.4 + key.4 <= 5
    }
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().next().unwrap() != "....." {
            return Err(anyhow::anyhow!("Invalid key"));
        }
        let mut schematic_block = s
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();
        schematic_block.reverse();

        Ok(Key::new(parse_block(schematic_block)))
    }
}

impl FromStr for Lock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().next().unwrap() != "#####" {
            return Err(anyhow::anyhow!("Invalid lock"));
        }
        let schematic_block = s
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();

        Ok(Lock::new(parse_block(schematic_block)))
    }
}

fn parse_block(block: Vec<Vec<char>>) -> (u8, u8, u8, u8, u8) {
    let rows = transpose(block);
    // for each row, count the number of #s
    let mut schematic = (0, 0, 0, 0, 0);
    for (i, row) in rows.iter().enumerate() {
        let count = row.iter().filter(|&&c| c == '#').count() - 1;
        match i {
            0 => schematic.0 = count as u8,
            1 => schematic.1 = count as u8,
            2 => schematic.2 = count as u8,
            3 => schematic.3 = count as u8,
            4 => schematic.4 = count as u8,
            _ => unreachable!(),
        }
    }
    schematic
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day25.run_test1(), 3)
    }

    #[test]
    fn test_key_parsing() {
        let key = Key::from_str(
            r#".....
.....
.....
...#.
...##
.#.##
#####"#,
        )
        .unwrap();
        assert_eq!(key, Key(0, 1, 0, 3, 2));
    }

    #[test]
    fn test_lock_parsing() {
        let lock = Lock::from_str(
            r#"#####
.####
.##.#
.#..#
.#..#
.#..#
....."#,
        )
        .unwrap();
        assert_eq!(lock, Lock(0, 5, 2, 1, 5));
    }
}
