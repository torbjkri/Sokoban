use super::game_state::GameState;
use super::player::Player;
use super::types::Size;
use super::yarn::Yarn;
use super::wall::Wall;
use super::basket::Basket;

const BOARD_SIZE: Size = Size::new(800, 800);
const UNIT_SIZE: Size = Size::new(40, 40);
const BOARD_MARGINS: Size = Size::new(20, 20);

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
    fn render_walls(&mut self, walls: &Vec<Wall>);
    fn render_wall(&mut self, wall: &Wall);
    fn is_open(&self) -> bool;
    fn poll_events(&mut self) -> CanvasEvents;
    fn render_winning_text(&mut self, winning: bool);
}

pub use self::sfmlcanvas::SfmlCanvas;
mod sfmlcanvas;
