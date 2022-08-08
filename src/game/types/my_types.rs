use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position{x: self.x + other.x, y: self.y + other.y}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}

impl Size{
    pub const fn new(x: u32, y: u32) -> Self {
        Self {x, y}
    }
}
