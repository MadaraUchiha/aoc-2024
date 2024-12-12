use std::str::FromStr;

use crate::vector::Vec2;

pub struct VectorMap<T> {
    map: Vec<Vec<T>>,
    size: Vec2,
}

impl<T> VectorMap<T> {
    pub fn get(&self, pos: &Vec2) -> Option<&T> {
        if pos.contained_in(Vec2::new(0, 0), self.size) {
            Some(&self.map[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub fn iter(&self) -> VectorMapIterator<T> {
        VectorMapIterator {
            map: self,
            curr_pos: Vec2::new(0, 0),
        }
    }
}

impl FromStr for VectorMap<char> {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let width = map[0].len();
        let height = map.len();

        Ok(VectorMap {
            size: (width, height).into(),
            map,
        })
    }
}

pub struct VectorMapIterator<'a, T> {
    map: &'a VectorMap<T>,
    curr_pos: Vec2,
}

impl <'a, T> Iterator for VectorMapIterator<'a, T> {
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