use super::canvas::{Canvas};
use super::game_state::GameState;

pub struct Application<T: Canvas> {
    pub canvas: T,
    pub game_state: GameState,
}

impl<T: Canvas> Application<T> {
    pub fn run(&mut self) {
        while self.canvas.is_open() {
            self.canvas.render(&mut self.game_state);
        }
    }
}