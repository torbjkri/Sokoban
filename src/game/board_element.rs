use crate::game::types::{Position};

pub trait BoardElement {
    fn board_position(&self) -> Position;
    fn check_collision(&self, other: &dyn BoardElement) -> bool;
}
