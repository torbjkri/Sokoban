use super::board::Board;
use super::board_element::BoardElement;
use super::canvas::CanvasEvents;
use super::movable::Movable;
use super::player::Player;
use super::yarn::Yarn;
use super::basket::Basket;
use crate::game::types::{Position, Size};

pub struct GameState {
    pub board: Board,
    pub yarns: Vec<Yarn>,
    pub player: Player,
    pub baskets: Vec<Basket>,
}

impl GameState {
    pub fn new() -> Self {
        let mut yarns = Vec::new();
        yarns.push(Yarn::new(Position::new(1, 1)));
        yarns.push(Yarn::new(Position::new(4, 3)));
        yarns.push(Yarn::new(Position::new(7, 7)));

        let mut baskets = Vec::new();
        baskets.push(Basket::new(Position::new(3,3)));
        Self {
            board: Board::new(Size::new(8, 8)),
            yarns: yarns,
            player: Player::new(Position::new(0, 0)),
            baskets: baskets,
        }
    }

    pub fn update(&mut self, events: CanvasEvents) {
        if events.a_pressed {
            self.player.move_left();
        } else if events.w_pressed {
            self.player.move_up();
        } else if events.s_pressed {
            self.player.move_down();
        } else if events.d_pressed {
            self.player.move_right();
        }

        if !self.player_has_legal_position() {
            self.player.undo_last_action();
        }
    }

    fn player_has_legal_position(&mut self) -> bool {
        if self.board.is_element_outside(&self.player) {
            return false;
        }

        for i in 0..self.yarns.len() {
            if self.player.board_position() == self.yarns[i].board_position() {
                self.yarns[i].do_action(self.player.get_last_action());
                if !self.yarn_has_legal_position(i) {
                    self.yarns[i].undo_last_action();
                    return false;
                }
                return true
            }
        }
        true
    }

    fn yarn_has_legal_position(&mut self, idx: usize) -> bool {
        if self.board.is_element_outside(&self.yarns[idx]) {
            return false;
        }

        for (i, yarn) in self.yarns.iter().enumerate() {
            if i != idx {
                if self.yarns[idx].board_position() == yarn.board_position() {
                    return false;
                }
            }
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game_state(board: Board, yarns: Vec<Yarn>, player: Player, baskets: Vec<Basket>) -> GameState {
        GameState {
            board,
            yarns,
            player,
            baskets,
        }
    }

    #[test]
    fn test_player_outside_board_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![],
            Player::new(Position::new(1, 1)),
            vec![]
        );
        assert_eq!(state.player_has_legal_position(), false);
    }

    #[test]
    fn test_yarn_outside_board_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![Yarn::new(Position::new(1,1))],
            Player::new(Position::new(0, 0)),
            vec![]
        );
        assert_eq!(state.yarn_has_legal_position(0), false);
    }

    #[test]
    fn test_yarn_on_yarn_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![Yarn::new(Position::new(0,0)), Yarn::new(Position::new(0,0))],
            Player::new(Position::new(1, 1)),
            vec![]
        );
        assert_eq!(state.yarn_has_legal_position(0), false);
    }

    #[test]
    fn test_player_move_into_yarn_moves_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1,0))],
            Player::new(Position::new(0, 0)),
            vec![]
        );
        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        let mut events =  CanvasEvents::new();
        events.d_pressed = true;
        state.update(events);

        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        assert_eq!(state.player.board_position(), Position::new(1,0));
        assert_eq!(state.yarns[0].board_position(), Position::new(2,0));
    }

    #[test]
    fn test_player_move_into_yarn_moves_yarn_if_illegal_reset() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1,0)), Yarn::new(Position::new(2,0))],
            Player::new(Position::new(0, 0)),
            vec![]
        );
        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        let mut events =  CanvasEvents::new();
        events.d_pressed = true;
        state.update(events);

        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        assert_eq!(state.player.board_position(), Position::new(0,0));
        assert_eq!(state.yarns[0].board_position(), Position::new(1,0));
        assert_eq!(state.yarns[1].board_position(), Position::new(2,0));
    }
}
