use game::{Canvas, Application, SfmlCanvas};
use game::{GameState};
mod game;

fn main() {
    let mut sokoban = Application{
        canvas: SfmlCanvas::new(game::CANVAS_SIZE),
        game_state: GameState::level1(),
    };
    sokoban.run();
}
