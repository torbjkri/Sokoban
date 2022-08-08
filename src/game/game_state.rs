use super::board::Board;
use super::yarn::Yarn;
use super::player::Player;
use crate::game::types::{Position, Size};
use super::canvas::CanvasEvents;
use super::movable::Movable;

pub struct GameState {
    pub board: Board,
    pub yarns: Vec<Yarn>,
    pub player: Player
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
            player: Player::new(Position::new(0,0))
        }
    }

    pub fn update(&mut self, events: CanvasEvents) {
        if events.a_pressed {
            self.player.move_left();
        }
        else if events.w_pressed {
            self.player.move_up();
        }
        else if events.s_pressed {
            self.player.move_down();
        }
        else if events.d_pressed {
            self.player.move_right();
        }

        if self.board.is_element_outside(&self.player) {
            self.player.undo_last_move();
        }
    }
}
