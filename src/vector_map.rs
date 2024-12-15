use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::vector::Vec2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorMap<T> {
    map: Vec<Vec<T>>,
    size: Vec2,
}

impl<T> VectorMap<T> {
    pub fn new(size: Vec2, default: T) -> Self
    where
        T: Clone,
    {
        let map = vec![vec![default; size.x as usize]; size.y as usize];
        VectorMap { map, size }
    }
    pub fn get(&self, pos: &Vec2) -> Option<&T> {
        if pos.contained_in(Vec2::new(0, 0), self.size) {
            Some(&self.map[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: &Vec2, value: T) {
        self.map[pos.y as usize][pos.x as usize] = value;
    }

    pub fn iter(&self) -> VectorMapIterator<T> {
        VectorMapIterator {
            map: self,
            curr_pos: Vec2::new(0, 0),
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}

impl FromStr for VectorMap<char> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();

        let width = map[0].len();
        let height = map.len();

        Ok(VectorMap {
            size: (width, height).into(),
            map,
        })
    }
}

impl Display for VectorMap<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for &c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct VectorMapIterator<'a, T> {
    map: &'a VectorMap<T>,
    curr_pos: Vec2,
}

impl<'a, T> Iterator for VectorMapIterator<'a, T> {
    type Item = (Vec2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.curr_pos;
        self.curr_pos.x += 1;
        if self.curr_pos.x == self.map.size.x {
            self.curr_pos.x = 0;
            self.curr_pos.y += 1;
        }

        self.map.get(&pos).map(|v| (pos, v))
    }
}
