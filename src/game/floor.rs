use super::board_element::{BoardElement, BoardElementVariant};
use crate::game::types::Position;

#[derive(Copy, Clone)]
pub struct Floor {
    position: Position,
}

impl Floor {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl BoardElement for Floor {
    fn board_position(&self) -> Position {
        self.position
    }

    fn check_collision(&self, other: &dyn BoardElement) -> bool {
        self.board_position() == other.board_position()
    }

    fn variant(&self) -> BoardElementVariant {
        BoardElementVariant::Floor
    }
}
