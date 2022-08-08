use super::super::board_element::BoardElement;
use super::super::game_state::GameState;
use super::super::yarn::Yarn;
use super::super::player::Player;
use super::{Canvas, CanvasEvents, BOARD_MARGINS, UNIT_SIZE};
use crate::game::types::{Position, Size};
use sfml::{
    graphics::{RenderTarget, RenderWindow, Texture, Transformable},
    system::{Vector2f, Vector2u},
    window::{ContextSettings, Event, Key, Style, VideoMode},
    SfBox,
};

pub struct SfmlCanvas {
    window: RenderWindow,
    board_texture: SfBox<Texture>,
    box_texture: SfBox<Texture>,
    player_texture: SfBox<Texture>
}

impl Canvas for SfmlCanvas {
    fn new(size: Size) -> Self {
        let mut window = RenderWindow::new(
            VideoMode::new(size.x, size.y, 32),
            "SFML Window",
            Style::CLOSE,
            &ContextSettings::default(),
        );
        window.set_framerate_limit(60);

        let board_texture = load_texture_file("assets/board8by8.png");
        let box_texture = load_texture_file("assets/niceboxsprite.jpg");
        let player_texture = load_texture_file("assets/cola.jpg");
        SfmlCanvas {
            window,
            board_texture,
            box_texture,
            player_texture,
        }
    }

    fn poll_events(&mut self) -> CanvasEvents {
        let mut canvas_events = CanvasEvents {
            a_pressed: false,
            w_pressed: false,
            s_pressed: false,
            d_pressed: false,
        };
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyReleased { code, .. } => {
                    if let Key::ESCAPE = code {
                        self.window.close();
                    }
                    if let Key::A = code {
                        canvas_events.a_pressed = true;
                    }
                    if let Key::W = code {
                        canvas_events.w_pressed = true;
                    }
                    if let Key::S = code {
                        canvas_events.s_pressed = true;
                    }
                    if let Key::D = code {
                        canvas_events.d_pressed = true;
                    }
                }
                _ => {}
            }
        }
        canvas_events
    }

    fn render(&mut self, game_state: &mut GameState) {
        self.window.set_active(true);
        self.render_board();
        self.render_yarns(&game_state.yarns);
        self.render_player(&game_state.player);

        self.window.display();
    }

    fn render_board(&mut self) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.board_texture);
        sprite.set_scale(texture_scale(
            self.board_texture.size(),
            Vector2u::new(8, 8),
        ));
        sprite.set_position(vec2f(BOARD_MARGINS));

        self.window.draw(&sprite);
    }

    fn render_yarn(&mut self, yarn: &Yarn) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.box_texture);
        sprite.set_scale(texture_scale(self.box_texture.size(), Vector2u::new(1, 1)));
        sprite.set_position(canvas_position_from_board_position(yarn.board_position()));
        self.window.draw(&sprite);
    }
    fn render_yarns(&mut self, yarns: &Vec<Yarn>) {
        for yarn in yarns.iter() {
            self.render_yarn(yarn);
        }
    }

    fn render_player(&mut self, player: &Player) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.player_texture);
        sprite.set_scale(texture_scale(self.player_texture.size(), Vector2u::new(1, 1)));
        sprite.set_position(canvas_position_from_board_position(player.board_position()));
        self.window.draw(&sprite);
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

fn canvas_position_from_board_position(position: Position) -> Vector2f {
    let x = position.x as f32;
    let y = position.y as f32;
    vec2f(BOARD_MARGINS) + vec2f(UNIT_SIZE) * Vector2f::new(x, y)
}

fn vec2f(size: Size) -> Vector2f {
    let x = size.x as f32;
    let y = size.y as f32;
    Vector2f::new(x, y)
}

fn vector2u_to_vector2f(vec: Vector2u) -> Vector2f {
    let x = vec.x as f32;
    let y = vec.y as f32;
    Vector2f { x, y }
}

fn texture_scale(texture_size: Vector2u, units: Vector2u) -> Vector2f {
    let size = vec2f(UNIT_SIZE) * vector2u_to_vector2f(units);
    let texture_size = vector2u_to_vector2f(texture_size);
    size / texture_size
}

fn load_texture_file(filename: &str) -> SfBox<Texture> {
    let texture = match Texture::from_file(filename) {
        None => panic!("Unable to read file {}", filename),
        Some(x) => x,
    };
    texture
}
