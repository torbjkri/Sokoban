use super::board::Board;
use super::board_element::BoardElement;
use super::canvas::CanvasEvents;
use super::movable::Movable;
use super::player::Player;
use super::yarn::Yarn;
use crate::game::types::{Position, Size};

pub struct GameState {
    pub board: Board,
    pub yarns: Vec<Yarn>,
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        let mut yarns = Vec::new();
        yarns.push(Yarn::new(Position::new(1, 1)));
        yarns.push(Yarn::new(Position::new(4, 3)));
        yarns.push(Yarn::new(Position::new(7, 7)));
        Self {
            board: Board::new(Size::new(8, 8)),
            yarns: yarns,
            player: Player::new(Position::new(0, 0)),
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

        if self.board.is_element_outside(&self.player) {
            self.player.undo_last_action();
        }

        resolve_collisions();
        if self.player_collide_with_yarn() {
            // self.player.undo_last_action();
            self.resolve_collisions();
        }
    }

    fn resolve_collisions(&mut self) {
        for i in 0..self.yarns.len() {
            if self.player.check_collision(&self.yarns[i]) {
                self.yarns[i].do_action(self.player.get_last_action());
                if !resolve_yarn_collisions() {
                    self.player.undo_last_action();
                }
                return;
            }
        }
    }

    fn resolve_yarn_collisions(&mut self) {
        for i in 0..self.yarns.len() {
            if self.yarn_collide_with_other_yarns(i) {
                self.yarns[i].
            }
        }
    }

    fn player_collide_with_yarn(&mut self) -> bool {
        for yarn in &mut self.yarns {
            if self.player.check_collision(yarn) {
                yarn.do_action(self.player.get_last_action());
                return true;
            }
        }
        false
    }

    fn yarn_collide_with_other_yarns(&mut self, idx: usize) -> bool {
        for (i, yarn) in self.yarns.iter().enumerate() {
            if i != idx {
                if self.yarns[idx].check_collision(yarn) {
                    return true;
                }
            }
        }
        false
    }

    fn resolve_collisions(&mut self) {
        for i in 0..self.yarns.len() {
            if self.yarn_collide_with_other_yarns(i) {
                
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game_state(board: Board, yarns: Vec<Yarn>, player: Player) -> GameState {
        GameState {
            board,
            yarns,
            player,
        }
    }

    #[test]
    fn test_player_collide_with_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![Yarn::new(Position::new(1, 1))],
            Player::new(Position::new(1, 1)),
        );
        assert_eq!(state.player_collide_with_yarn(), true);
    }

    #[test]
    fn test_player_move_into_yarn_move_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1, 0))],
            Player::new(Position::new(0, 0)),
        );
        assert_eq!(state.player_collide_with_yarn(), false);
        state.player.move_right();
        assert_eq!(state.player_collide_with_yarn(), true);
        assert_eq!(state.yarns[0].board_position(), Position::new(2,1));
    }

    #[test]
    fn test_yarn_collide_with_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(2, 1)),
            vec![Yarn::new(Position::new(1, 0)), Yarn::new(Position::new(1,0))],
            Player::new(Position::new(0, 0)),
        );
        assert_eq!(state.yarn_collide_with_other_yarns(0), true);
    }
}
