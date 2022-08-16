use super::game_state::GameState;
use super::types::Size;
use super::board_element::BoardElement;

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
    fn render_board_elements<T: BoardElement>(&mut self, elements: &Vec<T>);
    fn render_board_element(&mut self, element: &dyn BoardElement);
    fn is_open(&self) -> bool;
    fn poll_events(&mut self) -> CanvasEvents;
    fn render_winning_text(&mut self, winning: bool);
}

pub use self::sfmlcanvas::SfmlCanvas;
mod sfmlcanvas;
