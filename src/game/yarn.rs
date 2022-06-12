use super::board::{BoardElement, Size, Position};

pub struct Yarn {
    position: Position,
    size: Size
}

impl Yarn {
    pub fn new() -> Self {
        Self{
            position: Position{x:0,y:0},
            size: Size{x: 1, y: 1}
        }
    }
    fn set_position(&mut self, position: Position) {
        self.position = position; 
    }
}

impl BoardElement for Yarn {
    fn board_position(&self) -> Position {
        self.position
    }
    fn size(&self) -> Size {
        self.size
    }
}