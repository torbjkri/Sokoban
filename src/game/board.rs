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

pub const BOARD_SIZE: Size = Size{x:640, y:640};
const UNIT_SIZE: Size = Size{x:60, y:60};

pub trait BoardElement {
    fn board_position(&self) -> Position;
    fn size(&self) -> Size;
}

pub struct Board {
}

impl Board {
    pub fn new() -> Self {
        Self{}
    }
}

impl BoardElement for Board {
    fn board_position(&self) -> Position {
        Position{x:0,y:0}
    }
    fn size(&self) -> Size {
        Size{x: BOARD_SIZE.x / UNIT_SIZE.x,
            y: BOARD_SIZE.y / UNIT_SIZE.y}
    }
}
