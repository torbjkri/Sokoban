
pub struct Position {
    x: u8,
    y: u8,
}

pub struct Size {
    x: u8,
    y: u8,
}

pub const BOARD_SIZE: Size = Size{640, 640};
const UNIT_SIZE: Size = Size{60, 60};

pub trait BoardElement {
    fn board_position() -> Position;
    fn size() -> Size;
}

pub struct Board {
}

impl Board {
    pub fn new() -> Self {
        Self{}
    }
}

impl BoardElement for Board {
    fn board_position() -> Position {
        Position{0,0}
    }
    fn size() -> Size {
        BOARD_SIZE / UNIT_SIZE
    }
}
