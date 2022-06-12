use game::{Canvas, Application, SfmlCanvas};
use game::{GameState};
mod game;

fn main() {
    let mut sokoban = Application{
        canvas: SfmlCanvas::new(),
        game_state: GameState::new(),
    };
    sokoban.run();
}
