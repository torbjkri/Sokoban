use super::board_element::BoardElement;
use super::movable::{Movable, Move};
use super::collidable::Collidable;
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

impl Collidable for Player {
    fn check_collision(&self, other: &dyn Collidable) -> bool {
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let player = Player::new(Position::new(5,5));
        assert_eq!(player.position,  Position::new(5,5));
        assert_eq!(player.size, Size::new(1,1));
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
    fn test_undow_last_move() {
        let mut player = Player::new(Position::new(5,5));
        player.move_up();
        assert_eq!(player.position,  Position::new(5,4));
        player.undo_last_move();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_undoing_twice_does_nothing() {
        let mut player = Player::new(Position::new(5,5));
        player.move_up();
        assert_eq!(player.position,  Position::new(5,4));
        player.undo_last_move();
        assert_eq!(player.position,  Position::new(5,5));
        player.undo_last_move();
        assert_eq!(player.position,  Position::new(5,5));
    }

    #[test]
    fn test_undoing_before_moving_does_nothing() {
        let mut player = Player::new(Position::new(5,5));
        player.undo_last_move();
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

