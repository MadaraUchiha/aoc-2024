use std::ops::{Add, Sub};

pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn _manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn _square_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    pub fn _direction(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            &UP => RIGHT,
            &RIGHT => DOWN,
            &DOWN => LEFT,
            &LEFT => UP,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn contained_in(&self, min: Self, max: Self) -> bool {
        self.x >= min.x && self.x < max.x && self.y >= min.y && self.y < max.y
    }

    pub fn get_surrounding_4(&self) -> [Self; 4] {
        [
            *self + UP,
            *self + DOWN,
            *self + LEFT,
            *self + RIGHT,
        ]
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}
