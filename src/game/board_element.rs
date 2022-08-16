use crate::game::types::{Position};

pub enum BoardElementVariant {
    Player,
    Yarn,
    Wall,
    Basket,
    Floor,
}

pub trait BoardElement {
    fn board_position(&self) -> Position;
    fn check_collision(&self, other: &dyn BoardElement) -> bool;
    fn variant(&self) -> BoardElementVariant;
}
