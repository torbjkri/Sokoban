use super::game_state::GameState;
use super::player::Player;
use super::types::Size;
use super::yarn::Yarn;
use super::basket::Basket;

const BOARD_SIZE: Size = Size::new(480, 480);
const UNIT_SIZE: Size = Size::new(60, 60);
const BOARD_MARGINS: Size = Size::new(40, 60);

pub struct CanvasEvents {
    pub a_pressed: bool,
    pub w_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
}

impl CanvasEvents {
    pub fn new() -> Self {
        Self {
            a_pressed: false,
            w_pressed: false,
            s_pressed: false,
            d_pressed: false,
        }
    }
}

pub trait Canvas {
    fn new(size: Size) -> Self;
    fn render(&mut self, game_state: &mut GameState);
    fn render_yarn(&mut self, yarn: &Yarn);
    fn render_yarns(&mut self, yarn: &Vec<Yarn>);
    fn render_basket(&mut self, yarn: &Basket);
    fn render_baskets(&mut self, yarn: &Vec<Basket>);
    fn render_player(&mut self, yarn: &Player);
    fn render_board(&mut self);
    fn is_open(&self) -> bool;
    fn poll_events(&mut self) -> CanvasEvents;
}

pub use self::sfmlcanvas::SfmlCanvas;
mod sfmlcanvas;
