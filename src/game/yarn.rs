use super::board_element::{BoardElement, BoardElementVariant};
use super::movable::{Movable, Move};
use crate::game::types::{Position};

#[derive(Copy, Clone)]
pub struct Yarn {
    position: Position,
    last_action: Move,
}

impl Yarn {
    pub fn new(position: Position) -> Self {
        Self{
            position: position,
            last_action: Move::None,
        }
    }
}

impl BoardElement for Yarn {
    fn board_position(&self) -> Position {
        self.position
    }

    fn check_collision(&self, other: &dyn BoardElement) -> bool {
        self.board_position() == other.board_position()
    }

    fn variant(&self) -> BoardElementVariant {
        BoardElementVariant::Yarn
    }
}

impl Movable for Yarn {
    fn move_up(&mut self) {
        self.position.y = self.position.y - 1;
        self.last_action = Move::Up;
    }
    fn move_down(&mut self) {
        self.position.y = self.position.y + 1;
        self.last_action = Move::Down;
    }
    fn move_left(&mut self) {
        self.position.x = self.position.x - 1;
        self.last_action = Move::Left;
    }
    fn move_right(&mut self) {
        self.position.x = self.position.x + 1;
        self.last_action = Move::Right;
    }
    fn undo_last_action(&mut self) {
        match self.last_action {
            Move::Up => {
                self.move_down();
                self.last_action = Move::None;
            }
            Move::Down => {
                self.move_up();
                self.last_action = Move::None;
            }
            Move::Left => {
                self.move_right();
                self.last_action = Move::None;
            }
            Move::Right => {
                self.move_left();
                self.last_action = Move::None;
            }
            _ => {}
        }
    }

    fn get_last_action(&self) -> Move {
        self.last_action
    }

    fn do_action(&mut self, action: Move) {
        match action {
            Move::Up => {
                self.move_up();
            }
            Move::Down => {
                self.move_down();
            }
            Move::Left => {
                self.move_left();
            }
            Move::Right => {
                self.move_right();
            }
            _ => {}
        }
        self.last_action = action;
    }
}
