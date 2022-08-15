use super::super::board_element::BoardElement;
use super::super::game_state::GameState;
use super::super::yarn::Yarn;
use super::super::basket::Basket;
use super::super::wall::Wall;
use super::super::player::Player;
use super::{Canvas, CanvasEvents, BOARD_MARGINS, UNIT_SIZE};
use crate::game::types::{Position, Size};
use sfml::{
    graphics::{RenderTarget, RenderWindow, Texture, Transformable, Font, Text, Color},
    system::{Vector2f, Vector2u},
    window::{ContextSettings, Event, Key, Style, VideoMode},
    SfBox,
};

pub struct SfmlCanvas {
    window: RenderWindow,
    board_texture: SfBox<Texture>,
    box_texture: SfBox<Texture>,
    player_texture: SfBox<Texture>,
    basket_texture: SfBox<Texture>,
    wall_texture: SfBox<Texture>,
    font: SfBox<Font>,
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
        let basket_texture = load_texture_file("assets/pant-kasse.jpg");
        let wall_texture = load_texture_file("assets/fake.jpg");
        let find_font = || -> SfBox<Font> {
            match Font::from_file("assets/LEMONMILK-Bold.otf") {
                None => panic!("Unable to read font!"),
                Some(font) => return font,
            }
        };
        let font = find_font();

        SfmlCanvas {
            window,
            board_texture,
            box_texture,
            player_texture,
            basket_texture,
            wall_texture,
            font,
        }
    }

    fn poll_events(&mut self) -> CanvasEvents {
        let mut canvas_events = CanvasEvents::new();
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
        self.window.clear(Color::BLACK);
        self.window.set_active(true);
        self.render_board();
        self.render_baskets(&game_state.baskets);
        self.render_yarns(&game_state.yarns);
        self.render_player(&game_state.player);
        self.render_walls(&game_state.walls);
        self.render_winning_text(game_state.winning());

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

    fn render_basket(&mut self, basket: &Basket) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.basket_texture);
        sprite.set_scale(texture_scale(self.basket_texture.size(), Vector2u::new(1, 1)));
        sprite.set_position(canvas_position_from_board_position(basket.board_position()));
        self.window.draw(&sprite);
    }
    fn render_baskets(&mut self, basket: &Vec<Basket>) {
        for basket in basket.iter() {
            self.render_basket(basket);
        }
    }

    fn render_player(&mut self, player: &Player) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.player_texture);
        sprite.set_scale(texture_scale(self.player_texture.size(), Vector2u::new(1, 1)));
        sprite.set_position(canvas_position_from_board_position(player.board_position()));
        self.window.draw(&sprite);
    }

    fn render_wall(&mut self, wall: &Wall) {
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.wall_texture);
        sprite.set_scale(texture_scale(self.wall_texture.size(), Vector2u::new(1, 1)));
        sprite.set_position(canvas_position_from_board_position(wall.board_position()));
        self.window.draw(&sprite);
    }
    fn render_walls(&mut self, walls: &Vec<Wall>) {
        for wall in walls.iter() {
            self.render_wall(wall);
        }
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn render_winning_text(&mut self, winning: bool) {
        if winning {
            let mut text = Text::new("FUCK YEAH\nYOU WIN!!", &self.font, 34);
            text.set_position(Vector2f::new(540.0,100.0));
            self.window.draw(&text);
            
        }
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
