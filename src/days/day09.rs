use std::str::FromStr;

use crate::solution::Solution;

pub struct Day09;

impl Solution for Day09 {
    type Answer = i64;
    fn day(&self) -> u8 {
        9
    }

    fn part1(input: &str) -> anyhow::Result<i64> {
        let mut disk = input.parse::<Disk>()?;
        disk.compact();
        Ok(disk.checksum() as i64)
    }

    fn part2(input: &str) -> anyhow::Result<i64> {
        let mut disk = input.parse::<Disk>()?;
        disk.compact_full_files();
        Ok(disk.checksum())
    }
}

#[derive(Debug)]
struct Disk {
    data: Vec<i32>,
}

impl Disk {
    fn compact(&mut self) {
        let mut end_ptr = self.data.len() - 1;
        let mut start_ptr = 0;
        while start_ptr < end_ptr {
            if self.data[start_ptr] != -1 {
                start_ptr += 1;
            } else if self.data[end_ptr] == -1 {
                end_ptr -= 1;
            } else {
                self.data.swap(start_ptr, end_ptr);
            }
        }
    }

    fn compact_full_files(&mut self) {
        let mut end_ptr = self.data.len() - 1;
        let mut current_file = &self.data[end_ptr];
        let mut latest_handled_file = *current_file;
        while end_ptr > 0 {
            if *current_file == -1 {
                end_ptr -= 1;
                current_file = &self.data[end_ptr];
                continue;
            }
            // Found a non-space, find the length of the file
            let mut file_length = 0;
            while &self.data[end_ptr - file_length] == current_file {
                file_length += 1;
                if end_ptr - file_length == 0 {
                    break;
                }
            }

            // Have I moved this file already?
            if *current_file > latest_handled_file {
                end_ptr -= file_length;
                current_file = &self.data[end_ptr];
                continue;
            }

            // Have all the information, now find a contiguous space to move the file to
            let mut space_cursor = 0;
            while let Some((space_index, space_length)) =
                self.find_next_contiguous_space(space_cursor)
            {
                // We've passed the file, search is over, move to the next file
                if space_index > end_ptr {
                    end_ptr -= file_length;
                    current_file = &self.data[end_ptr];
                    break;
                }

                // The space is too small, continue searching
                if space_length < file_length {
                    space_cursor = space_index + space_length;
                    continue;
                }

                // Found a space that fits the file, move the file
                for i in 0..file_length {
                    self.data.swap(end_ptr - i, space_index + i);
                }

                // Move the cursor to the beginning of the moved space, and continue to next file
                latest_handled_file -= 1;
                end_ptr -= file_length;
                current_file = &self.data[end_ptr];
                break;
            }
        }
    }

    fn find_next_contiguous_space(&self, start: usize) -> Option<(usize, usize)> {
        let mut cursor = start;
        while self.data[cursor] != -1 {
            cursor += 1;
        }
        if cursor == self.data.len() {
            return None;
        }
        let mut length = 0;
        while self.data[cursor + length] == -1 {
            if cursor + length + 1 == self.data.len() {
                return Some((cursor, length));
            }
            length += 1;
        }
        Some((cursor, length))
    }

    fn checksum(&self) -> i64 {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != -1)
            .map(|(i, &v)| i as i64 * v as i64)
            .sum()
    }
}

impl FromStr for Disk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::with_capacity(s.len() * 9);
        let mut id = 0;
        let mut is_file = true;
        for c in s.chars() {
            let length = c.to_digit(10).unwrap() as usize;
            if is_file {
                data.extend_from_slice(&[id].repeat(length));
                id += 1;
            } else {
                data.extend_from_slice(&[-1].repeat(length));
            }
            is_file = !is_file;
        }

        Ok(Disk { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let day = Day09;
        assert_eq!(day.run_test1(), 1928);
    }

    #[test]
    fn part2() {
        let day = Day09;
        assert_eq!(day.run_test2(), 2858);
    }

    #[test]
    fn compact() {
        let mut disk = "12345".parse::<Disk>().unwrap();
        disk.compact();
        assert_eq!(
            disk.data,
            vec![0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1]
        );
    }

    #[test]
    fn more_compact() {
        let mut disk = "2333133121414131402".parse::<Disk>().unwrap();
        disk.compact();
        assert_eq!(
            disk.data,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
            ]
        );
    }

    #[test]
    fn compact_full() {
        let mut disk = "2333133121414131402".parse::<Disk>().unwrap();
        disk.compact_full_files();
        assert_eq!(
            disk.data,
            vec![
                0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5,
                -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1
            ]
        );
    }

    #[test]
    fn checksum() {
        let mut disk = "12345".parse::<Disk>().unwrap();
        disk.compact();
        assert_eq!(disk.checksum(), 0 + 2 + 4 + 3 + 4 + 5 + 12 + 14 + 16);
    }
}
