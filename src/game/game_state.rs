use super::board::Board;
use super::yarn::Yarn;
use crate::game::types::{Position, Size};
use super::canvas::CanvasEvents;
use super::movable::Movable;

pub struct GameState {
    pub board: Board,
    pub yarns: Vec<Yarn>
}

impl GameState {
    pub fn new() -> Self {
        let mut yarns = Vec::new();
        yarns.push(Yarn::new(Position::new(1,1)));
        yarns.push(Yarn::new(Position::new(4,3)));
        yarns.push(Yarn::new(Position::new(7,7)));
        Self {
            board: Board::new(Size::new(8,8)),
            yarns: yarns,
        }
    }

    pub fn update(&mut self, events: CanvasEvents) {
        if events.a_pressed {
            self.yarns[0].move_left();
        }
        else if events.w_pressed {
            self.yarns[0].move_up();
        }
        else if events.s_pressed {
            self.yarns[0].move_down();
        }
        else if events.d_pressed {
            self.yarns[0].move_right();
        }
    }
}
