use sfml::{
    graphics::{RenderWindow, RenderTarget, Texture, Transformable},
    window::{ContextSettings,Style, VideoMode, Event, Key},
    SfBox,
    system::{Vector2f, Vector2u},
};
use super::game_state::GameState;
use super::yarn::Yarn;
use super::board_element::BoardElement;
use super::movable::Movable;
use crate::game::types::Position;

const CANVAS_SIZE: Vector2u = Vector2u::new(800, 600);
const BOARD_SIZE: Vector2u = Vector2u::new(480, 480);
const UNIT_SIZE: Vector2f = Vector2f::new(60.0, 60.0);
const BOARD_MARGINS: Vector2f = Vector2f::new(40.0, 60.0);

pub trait Canvas {
    fn new() -> Self;
    fn render(&mut self, game_state: &mut GameState);
    fn render_yarn(&mut self, yarn: &Yarn);
    fn render_yarns(&mut self, yarn: &Vec<Yarn>);
    fn render_board(&mut self);
    fn is_open(&self) -> bool;
}

pub struct SfmlCanvas {
    window: RenderWindow,
    board_texture: SfBox<Texture>,
    box_texture: SfBox<Texture>
}

impl Canvas for SfmlCanvas {
    fn new() -> Self {
        let mut window = RenderWindow::new(
            VideoMode::new(CANVAS_SIZE.x, CANVAS_SIZE.y, 32),
            "SFML Window",
            Style::CLOSE,
            &ContextSettings::default());
        window.set_framerate_limit(60);

        let board_texture = load_texture_file("assets/board8by8.png");
        let box_texture = load_texture_file("assets/niceboxsprite.jpg");
        SfmlCanvas{ 
            window,
            board_texture,
            box_texture,
        }
    }

    fn render(&mut self, game_state: &mut GameState) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyReleased {code, ..} => {
                    if let Key::ESCAPE = code {
                        self.window.close();
                    }
                    if let Key::A = code {
                        game_state.yarns[0].move_up();
                    }
                },
                _ => {}
            }
        }
        

        self.window.set_active(true);
        self.render_board();
        self.render_yarns(&game_state.yarns);

        self.window.display();
    }

    fn render_board(&mut self) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.board_texture);
        sprite.set_scale(texture_scale(self.board_texture.size(), Vector2u::new(8,8)));
        sprite.set_position(BOARD_MARGINS);

        self.window.draw(&sprite);
    }

    fn render_yarn(&mut self, yarn: &Yarn) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.box_texture);
        sprite.set_scale(texture_scale(self.box_texture.size(), Vector2u::new(1,1)));
        sprite.set_position(canvas_position_from_board_position(yarn.board_position()));
        self.window.draw(&sprite);
    }
    
    fn render_yarns(&mut self, yarns: &Vec<Yarn>) {
        for yarn in yarns.iter() {
            self.render_yarn(yarn);
        }
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

fn canvas_position_from_board_position(position: Position) -> Vector2f {
    let x =  position.x as f32;
    let y = position.y as f32;
    BOARD_MARGINS + UNIT_SIZE * Vector2f::new(x, y)
}

fn vector2u_to_vector2f(vec: Vector2u) -> Vector2f {
    let x = vec.x as f32;
    let y = vec.y as f32;
    Vector2f{x, y}
}

fn texture_scale(texture_size: Vector2u, units: Vector2u) -> Vector2f {
    let size = UNIT_SIZE * vector2u_to_vector2f(units);
    let texture_size = vector2u_to_vector2f(texture_size);
    size /texture_size
}

fn load_texture_file(filename: &str) -> SfBox<Texture> {
    let texture = match Texture::from_file(filename) {
        None => panic!("Unable to read file {}", filename),
        Some(x) => x,
    };
    texture
}
