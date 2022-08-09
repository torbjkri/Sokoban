use super::board_element::{BoardElement};
use super::movable::{Movable, Move};
use crate::game::types::{Position, Size};

pub struct Yarn {
    position: Position,
    size: Size,
    last_move: Move,
}

impl Yarn {
    pub fn new(position: Position) -> Self {
        Self{
            position: position,
            size: Size{x: 1, y: 1},
            last_move: Move::None,
        }
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
        self.last_move = Move::Up;
    }
    fn move_down(&mut self) {
        self.position.y = self.position.y + 1;
        self.last_move = Move::Down;
    }
    fn move_left(&mut self) {
        self.position.x = self.position.x - 1;
        self.last_move = Move::Left;
    }
    fn move_right(&mut self) {
        self.position.x = self.position.x + 1;
        self.last_move = Move::Right;
    }
    fn undo_last_move(&mut self) {
        match self.last_move {
            Move::Up => {
                self.move_down();
                self.last_move = Move::None;
            }
            Move::Down => {
                self.move_up();
                self.last_move = Move::None;
            }
            Move::Left => {
                self.move_right();
                self.last_move = Move::None;
            }
            Move::Right => {
                self.move_left();
                self.last_move = Move::None;
            }
            _ => {}
        }
    }
}
