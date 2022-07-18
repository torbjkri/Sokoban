use super::board_element::{BoardElement};
use super::movable::{Movable};
use crate::game::types::{Position, Size};

pub struct Yarn {
    position: Position,
    size: Size
}

impl Yarn {
    pub fn new(position: Position) -> Self {
        Self{
            position: position,
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

impl Movable for Yarn {
    fn move_up(&mut self) {
        self.position.y = self.position.y - 1;
    }
    fn move_down(&mut self) {
        self.position.y = self.position.y + 1;
    }
    fn move_left(&mut self) {
        self.position.x = self.position.x - 1;
    }
    fn move_right(&mut self) {
        self.position.x = self.position.x + 1;
    }
}