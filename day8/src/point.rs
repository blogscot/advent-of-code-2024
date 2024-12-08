use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Point {
    pub fn new(row: i32, col: i32) -> Point {
        Point { row, col }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}
