use super::canvas::{Canvas};
use super::game_state::GameState;
use super::types::Size;

pub const CANVAS_SIZE: Size = Size::new(900, 840);

pub struct Application<T: Canvas> {
    pub canvas: T,
    pub game_state: GameState,
}

impl<T: Canvas> Application<T> {
    pub fn run(&mut self) {
        while self.canvas.is_open() {
            let events = self.canvas.poll_events();
            self.game_state.update(events);
            self.canvas.render(&mut self.game_state);
        }
    }
}
