use super::basket::Basket;
use super::board::Board;
use super::board_element::BoardElement;
use super::canvas::CanvasEvents;
use super::movable::Movable;
use super::player::Player;
use super::yarn::Yarn;
use super::wall::Wall;
use super::floor::Floor;
use crate::game::types::{Position, Size};

pub struct GameState {
    pub board: Board,
    pub yarns: Vec<Yarn>,
    pub player: Player,
    pub baskets: Vec<Basket>,
    pub walls: Vec<Wall>,
    pub floors: Vec<Floor>,
}

impl GameState {
    pub fn new() -> Self {
        let mut yarns = Vec::new();
        yarns.push(Yarn::new(Position::new(1, 1)));
        yarns.push(Yarn::new(Position::new(4, 3)));

        let mut baskets = Vec::new();
        baskets.push(Basket::new(Position::new(3, 4)));
        baskets.push(Basket::new(Position::new(6, 7)));

        let mut walls = Vec::new();
        walls.push(Wall::new(Position::new(6,6)));
        walls.push(Wall::new(Position::new(2,2)));
        Self {
            board: Board::new(Size::new(8, 8)),
            yarns: yarns,
            player: Player::new(Position::new(0, 0)),
            baskets: baskets,
            walls: walls,
            floors: vec![],
        }
    }

    pub fn level1() -> Self {
        let mut walls = Vec::new();
        walls.push(Wall::new(Position::new(0,0)));
        walls.push(Wall::new(Position::new(1,0)));
        walls.push(Wall::new(Position::new(2,0)));
        walls.push(Wall::new(Position::new(3,0)));
        walls.push(Wall::new(Position::new(4,0)));
        walls.push(Wall::new(Position::new(0,1)));
        walls.push(Wall::new(Position::new(4,1)));
        walls.push(Wall::new(Position::new(0,2)));
        walls.push(Wall::new(Position::new(2,2)));
        walls.push(Wall::new(Position::new(4,2)));
        walls.push(Wall::new(Position::new(5,2)));
        walls.push(Wall::new(Position::new(6,2)));
        walls.push(Wall::new(Position::new(0,3)));
        walls.push(Wall::new(Position::new(6,3)));
        walls.push(Wall::new(Position::new(0,4)));
        walls.push(Wall::new(Position::new(1,4)));
        walls.push(Wall::new(Position::new(2,4)));
        walls.push(Wall::new(Position::new(3,4)));
        walls.push(Wall::new(Position::new(4,4)));
        walls.push(Wall::new(Position::new(5,4)));
        walls.push(Wall::new(Position::new(6,4)));

        let mut floors = Vec::new();
        floors.push(Floor::new(Position::new(1,1)));
        floors.push(Floor::new(Position::new(2,1)));
        floors.push(Floor::new(Position::new(3,1)));
        floors.push(Floor::new(Position::new(1,2)));
        floors.push(Floor::new(Position::new(3,2)));
        floors.push(Floor::new(Position::new(1,3)));
        floors.push(Floor::new(Position::new(2,3)));
        floors.push(Floor::new(Position::new(3,3)));

        let mut yarns = Vec::new();
        yarns.push(Yarn::new(Position::new(3,2)));
        yarns.push(Yarn::new(Position::new(2,3)));

        let mut baskets = Vec::new();
        baskets.push(Basket::new(Position::new(4,3)));
        baskets.push(Basket::new(Position::new(5,3)));

        Self {
            board: Board::new(Size::new(8, 8)),
            yarns: yarns,
            player: Player::new(Position::new(1, 1)),
            baskets: baskets,
            walls: walls,
            floors: floors
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
                return true;
            }
        }

        for i in 0..self.walls.len() {
            if self.player.board_position() == self.walls[i].board_position() {
                return false;
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

        for wall in &self.walls {
            if self.yarns[idx].board_position() == wall.board_position() {
                return false;
            }
        }
        true
    }

    fn basket_has_yarn(&self, idx: usize) -> bool {
        for yarn in &self.yarns {
            if yarn.check_collision(&self.baskets[idx]) {
                return true;
            }
        }
        false
    }

    pub fn winning(&self) -> bool {
        for i in 0..self.baskets.len() {
            if !self.basket_has_yarn(i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game_state(
        board: Board,
        yarns: Vec<Yarn>,
        player: Player,
        baskets: Vec<Basket>,
        walls: Vec<Wall>
    ) -> GameState {
        GameState {
            board,
            yarns,
            player,
            baskets,
            walls,
        }
    }

    #[test]
    fn test_player_outside_board_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![],
            Player::new(Position::new(1, 1)),
            vec![],
            vec![],
        );
        assert_eq!(state.player_has_legal_position(), false);
    }

    #[test]
    fn test_yarn_outside_board_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![Yarn::new(Position::new(1, 1))],
            Player::new(Position::new(0, 0)),
            vec![],
            vec![],
        );
        assert_eq!(state.yarn_has_legal_position(0), false);
    }

    #[test]
    fn test_yarn_on_yarn_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(1, 1)),
            vec![
                Yarn::new(Position::new(0, 0)),
                Yarn::new(Position::new(0, 0)),
            ],
            Player::new(Position::new(1, 1)),
            vec![],
            vec![],
        );
        assert_eq!(state.yarn_has_legal_position(0), false);
    }

    #[test]
    fn test_player_move_into_yarn_moves_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1, 0))],
            Player::new(Position::new(0, 0)),
            vec![],
            vec![],
        );
        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        let mut events = CanvasEvents::new();
        events.d_pressed = true;
        state.update(events);

        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        assert_eq!(state.player.board_position(), Position::new(1, 0));
        assert_eq!(state.yarns[0].board_position(), Position::new(2, 0));
    }

    #[test]
    fn test_player_move_into_yarn_moves_yarn_if_illegal_reset() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![
                Yarn::new(Position::new(1, 0)),
                Yarn::new(Position::new(2, 0)),
            ],
            Player::new(Position::new(0, 0)),
            vec![],
            vec![],
        );
        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        let mut events = CanvasEvents::new();
        events.d_pressed = true;
        state.update(events);

        assert_eq!(state.player_has_legal_position(), true);
        assert_eq!(state.yarn_has_legal_position(0), true);
        assert_eq!(state.player.board_position(), Position::new(0, 0));
        assert_eq!(state.yarns[0].board_position(), Position::new(1, 0));
        assert_eq!(state.yarns[1].board_position(), Position::new(2, 0));
    }

    #[test]
    fn basket_detects_yarn() {
        let mut state = create_test_game_state(
            Board::new(Size::new(2, 1)),
            vec![Yarn::new(Position::new(1, 0))],
            Player::new(Position::new(0, 0)),
            vec![Basket::new(Position::new(1, 0))],
            vec![],
        );

        assert_eq!(state.basket_has_yarn(0), true);
    }

    #[test]
    fn test_not_winning() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1, 0))],
            Player::new(Position::new(0, 0)),
            vec![Basket::new(Position::new(2, 0))],
            vec![],
        );

        assert_eq!(state.winning(), false);
    }

    #[test]
    fn test_winning() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(2, 0)), Yarn::new(Position::new(1,0))],
            Player::new(Position::new(0, 0)),
            vec![Basket::new(Position::new(2, 0))],
            vec![],
        );

        assert_eq!(state.winning(), true);
    }

    #[test]
    fn test_player_on_wall_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![],
            Player::new(Position::new(0, 0)),
            vec![],
            vec![Wall::new(Position::new(0,0))],
        );

        assert_eq!(state.player_has_legal_position(), false);
    }

    #[test]
    fn test_yarn_on_wall_illegal() {
        let mut state = create_test_game_state(
            Board::new(Size::new(3, 1)),
            vec![Yarn::new(Position::new(1,0))],
            Player::new(Position::new(0, 0)),
            vec![],
            vec![Wall::new(Position::new(1,0))],
        );

        assert_eq!(state.yarn_has_legal_position(0), false);
    }
}
