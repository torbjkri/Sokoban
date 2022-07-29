use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self {x, y}
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position{x: self.x + other.x, y: self.y + other.y}
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}

impl Size{
    pub const fn new(x: u32, y: u32) -> Self {
        Self {x, y}
    }
}
