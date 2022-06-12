use super::board::Board;
use super::yarn::Yarn;

pub struct GameState {
    board: Board,
    yarn: Yarn
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            yarn: Yarn::new(),
        }
    }
}