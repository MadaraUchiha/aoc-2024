use std::{
    fmt::Display,
    ops::{Add, Mul, Rem, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    const ZERO: Self = Vec2 { x: 0, y: 0 };

    pub const UP: Self = Vec2 { x: 0, y: -1 };
    pub const DOWN: Self = Vec2 { x: 0, y: 1 };
    pub const LEFT: Self = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Self = Vec2 { x: 1, y: 0 };

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn _square_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    pub fn _direction(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            &Self::UP => Self::RIGHT,
            &Self::RIGHT => Self::DOWN,
            &Self::DOWN => Self::LEFT,
            &Self::LEFT => Self::UP,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        match self {
            &Self::UP => Self::LEFT,
            &Self::LEFT => Self::DOWN,
            &Self::DOWN => Self::RIGHT,
            &Self::RIGHT => Self::UP,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn contained_in(&self, min: Self, max: Self) -> bool {
        self.x >= min.x && self.x < max.x && self.y >= min.y && self.y < max.y
    }

    pub fn get_surrounding_4(&self) -> [Self; 4] {
        [
            *self + Self::UP,
            *self + Self::DOWN,
            *self + Self::LEFT,
            *self + Self::RIGHT,
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

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
        }
    }
}

impl Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl FromStr for Vec2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}
