use crate::game::types::{Position, Size};
use super::board_element::BoardElement;

pub struct Board {
    size: Size
}

impl Board {
    pub fn new(size: Size) -> Self {
        Self{size}
    }

    pub fn is_element_outside(&mut self, element: &dyn BoardElement) -> bool {
        element.board_position().x < 0 || element.board_position().x >= (self.size.x as i32) || element.board_position().y < 0 || element.board_position().y >= (self.size.y as i32)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    struct MockElement {
        position: Position
    }

    impl MockElement {
        pub fn new(position: Position) -> Self {
            Self{position}
        }
    }

    impl BoardElement for MockElement {
        fn board_position(&self) -> Position {
            self.position
        }
        fn check_collision(&self, _other: &dyn BoardElement) -> bool {
            false
        }
    }

    #[test]
    fn test_legal_valid_inside() {
        let elem = MockElement::new(Position::new(2,2));
        let mut board = Board::new(Size::new(8,8));
        assert_eq!(board.is_element_outside(&elem), false);
    }

    #[test]
    fn test_large_position_invalid() {
        let elem = MockElement::new(Position::new(10,2));
        let mut board = Board::new(Size::new(8,8));
        assert_eq!(board.is_element_outside(&elem), true);
    }

    #[test]
    fn test_small_position_invalid() {
        let elem = MockElement::new(Position::new(-1,2));
        let mut board = Board::new(Size::new(8,8));
        assert_eq!(board.is_element_outside(&elem), true);
    }
}
