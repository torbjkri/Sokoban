use super::board_element::BoardElement;
use crate::game::types::Position;



#[derive(Copy, Clone)]
pub struct Basket {
    position: Position,
}

impl Basket {
    pub fn new(position: Position) -> Self {
        Self {
            position,
        }
    }
}

impl BoardElement for Basket {
    fn board_position(&self) -> Position {
        self.position
    }

    fn check_collision(&self, other: &dyn BoardElement) -> bool {
        self.board_position() == other.board_position()
    }
}
