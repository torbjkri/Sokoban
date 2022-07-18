use crate::game::types::{Position, Size};
use super::board_element::BoardElement;

pub struct Board {
    size: Size
}

impl Board {
    pub fn new(size: Size) -> Self {
        Self{size}
    }
}

impl BoardElement for Board {
    fn board_position(&self) -> Position {
        Position{x:0,y:0}
    }
    fn size(&self) -> Size {
        self.size
    }
}
