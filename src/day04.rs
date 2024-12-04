use std::str::FromStr;

use anyhow::Result;

use crate::solution::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn day(&self) -> u8 {
        4
    }

    fn part1(input: &str) -> Result<i64> {
        let board: WordSearchBoard = input.parse()?;

        let mut result = 0;
        for (x, y) in board.all_letters('X') {
            let options = board.find_xmas_options(x as isize, y as isize);

            result += options
                .iter()
                .filter(|word| word.as_str() == "XMAS")
                .count() as i64;
        }

        Ok(result)
    }

    fn part2(input: &str) -> Result<i64> {
        let board: WordSearchBoard = input.parse()?;

        let all_xmas = board
            .all_letters('A')
            .filter(|(x, y)| board.is_xmas(*x as isize, *y as isize))
            .count() as i64;

        Ok(all_xmas)
    }
}

struct WordSearchBoard(Vec<Vec<char>>);

impl WordSearchBoard {
    fn find_xmas_options(&self, x: isize, y: isize) -> Vec<String> {
        let forwards = vec![
            self.get(x, y),
            self.get(x + 1, y),
            self.get(x + 2, y),
            self.get(x + 3, y),
        ];
        let backwards = vec![
            self.get(x, y),
            self.get(x - 1, y),
            self.get(x - 2, y),
            self.get(x - 3, y),
        ];
        let up = vec![
            self.get(x, y),
            self.get(x, y - 1),
            self.get(x, y - 2),
            self.get(x, y - 3),
        ];
        let down = vec![
            self.get(x, y),
            self.get(x, y + 1),
            self.get(x, y + 2),
            self.get(x, y + 3),
        ];
        let up_right = vec![
            self.get(x, y),
            self.get(x + 1, y - 1),
            self.get(x + 2, y - 2),
            self.get(x + 3, y - 3),
        ];
        let up_left = vec![
            self.get(x, y),
            self.get(x - 1, y - 1),
            self.get(x - 2, y - 2),
            self.get(x - 3, y - 3),
        ];
        let down_right = vec![
            self.get(x, y),
            self.get(x + 1, y + 1),
            self.get(x + 2, y + 2),
            self.get(x + 3, y + 3),
        ];
        let down_left = vec![
            self.get(x, y),
            self.get(x - 1, y + 1),
            self.get(x - 2, y + 2),
            self.get(x - 3, y + 3),
        ];

        let all = vec![
            forwards, backwards, up, down, up_right, up_left, down_right, down_left,
        ];
        all.into_iter()
            .filter_map(|word| word.into_iter().collect())
            .collect()
    }

    fn is_xmas(&self, x: isize, y: isize) -> bool {
        let diagonal1 = vec![
            self.get(x - 1, y - 1),
            self.get(x, y),
            self.get(x + 1, y + 1),
        ];
        let diagonal2 = vec![
            self.get(x + 1, y - 1),
            self.get(x, y),
            self.get(x - 1, y + 1),
        ];

        let diagonal1_matches = match diagonal1.as_slice() {
            [Some('M'), Some('A'), Some('S')] => true,
            [Some('S'), Some('A'), Some('M')] => true,
            _ => false,
        };

        let diagonal2_matches = match diagonal2.as_slice() {
            [Some('M'), Some('A'), Some('S')] => true,
            [Some('S'), Some('A'), Some('M')] => true,
            _ => false,
        };

        diagonal1_matches && diagonal2_matches
    }

    fn all_letters(&self, letter: char) -> LetterFinder {
        LetterFinder::new(self, letter)
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x_usize = x as usize;
        let y_usize = y as usize;
        if y_usize >= self.0.len() {
            return None;
        }
        if x_usize >= self.0[0].len() {
            return None;
        }
        self.0
            .get(y_usize)
            .and_then(|row| row.get(x_usize))
            .copied()
    }
}

struct LetterFinder<'a> {
    board: &'a WordSearchBoard,
    letter: char,
    x: usize,
    y: usize,
}

impl<'a> LetterFinder<'a> {
    fn new(board: &'a WordSearchBoard, letter: char) -> Self {
        Self {
            board,
            letter,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for LetterFinder<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x, y) = (self.x, self.y);
            self.x += 1;
            if self.x > self.board.0[0].len() {
                self.x = 0;
                self.y += 1;
            }
            if self.y > self.board.0.len() {
                return None;
            }
            if self.board.get(x as isize, y as isize) == Some(self.letter) {
                return Some((x, y));
            }
        }
    }
}

impl FromStr for WordSearchBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WordSearchBoard(
            s.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day04.run_test1(), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(Day04.run_test2(), 9);
    }
}
