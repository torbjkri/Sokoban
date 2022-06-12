use super::board::{BoardElement, Size, Position};

pub struct Yarn {
    position: Position,
    size: Size
}

impl Yarn {
    fn new() -> Self {
        Self{
            Position{0,0},
            Size{1,1}
        }
    }
    fn set_position(&mut self, position: Position) {
        self.position = position; 
    }
}

impl BoardElement for Yarn {
    fn board_position(&self) -> Position {
        self.board_position
    }
    fn size(&self) -> Size {
        self.size
    }
}