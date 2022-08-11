use super::board_element::BoardElement;
use super::movable::{Movable, Move};
use crate::game::types::{Position};

pub struct Player {
    position: Position,
    last_action: Move,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            position: position,
            last_action: Move::None,
        }
    }
}

impl BoardElement for Player {
    fn board_position(&self) -> Position {
        self.position
    }

    fn check_collision(&self, other: &dyn BoardElement) -> bool {
        self.board_position() == other.board_position()
    }
}

impl Movable for Player {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let player = Player::new(Position::new(5,5));
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_moves() {
        let mut player = Player::new(Position::new(5,5));
        player.move_up();
        assert_eq!(player.position,  Position::new(5,4));
        player.move_left();
        assert_eq!(player.position,  Position::new(4,4));
        player.move_down();
        assert_eq!(player.position,  Position::new(4,5));
        player.move_right();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_undo_last_action() {
        let mut player = Player::new(Position::new(5,5));
        player.move_up();
        assert_eq!(player.position,  Position::new(5,4));
        player.undo_last_action();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_undoing_twice_does_nothing() {
        let mut player = Player::new(Position::new(5,5));
        player.move_up();
        assert_eq!(player.position,  Position::new(5,4));
        player.undo_last_action();
        assert_eq!(player.position,  Position::new(5,5));
        player.undo_last_action();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_undoing_before_moving_does_nothing() {
        let mut player = Player::new(Position::new(5,5));
        player.undo_last_action();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_non_collision_when_not_overlapping() {
        let player1 = Player::new(Position::new(5,5));
        let player2 = Player::new(Position::new(5,4));
        assert_eq!(player1.check_collision(&player2), false);
    }

    #[test]
    fn test_collision_when_overlapping() {
        let player1 = Player::new(Position::new(5,5));
        let player2 = Player::new(Position::new(5,5));
        assert_eq!(player1.check_collision(&player2), true);
    }
}

