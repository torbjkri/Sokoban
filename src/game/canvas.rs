use super::game_state::GameState;
use super::yarn::Yarn;
use super::types::Size;

const BOARD_SIZE: Size = Size::new(480, 480);
const UNIT_SIZE: Size = Size::new(60, 60);
const BOARD_MARGINS: Size = Size::new(40, 60);

pub struct CanvasEvents {
    pub a_pressed: bool,
    pub w_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool
}

pub trait Canvas {
    fn new(size: Size) -> Self;
    fn render(&mut self, game_state: &mut GameState);
    fn render_yarn(&mut self, yarn: &Yarn);
    fn render_yarns(&mut self, yarn: &Vec<Yarn>);
    fn render_board(&mut self);
    fn is_open(&self) -> bool;
    fn poll_events(&mut self) -> CanvasEvents;
}

pub use self::sfmlcanvas::SfmlCanvas;
mod sfmlcanvas;
