use super::board_element::BoardElement;
use super::movable::{Movable, Move};
use crate::game::types::{Position, Size};

pub struct Player {
    position: Position,
    size: Size,
    last_move: Move,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            position: position,
            size: Size { x: 1, y: 1 },
            last_move: Move::None,
        }
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

impl BoardElement for Player {
    fn board_position(&self) -> Position {
        self.position
    }
    fn size(&self) -> Size {
        self.size
    }
}

impl Movable for Player {
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
