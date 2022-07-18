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
    pub x: u16,
    pub y: u16,
}

impl Size{
    pub fn new(x: u16, y: u16) -> Self {
        Self {x, y}
    }
}