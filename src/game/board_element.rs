use crate::game::types::{Position, Size};

pub trait BoardElement {
    fn board_position(&self) -> Position;
    fn size(&self) -> Size;
}