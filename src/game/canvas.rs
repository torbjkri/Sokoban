use sfml::{
    graphics::{RenderWindow, RenderTarget, Texture, Transformable},
    window::{ContextSettings,Style, VideoMode, Event},
    SfBox,
    system::{Vector2f, Vector2u},
};

const CANVAS_SIZE: Vector2u = Vector2u::new(800, 600);
const BOARD_SIZE: Vector2u = Vector2u::new(480, 480);
const UNIT_SIZE: Vector2u = Vector2u::new(60, 60);
const BOARD_MARGINS: Vector2f = Vector2f::new(40.0, 60.0);

pub trait Canvas {
    fn new() -> Self;
    fn render(&mut self);
    fn is_open(&self) -> bool;
}

pub struct SfmlCanvas {
    window: RenderWindow,
    board_texture: SfBox<Texture>
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
        SfmlCanvas{ 
            window,
            board_texture,
        }
    }

    fn render(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close();
            } 
        }
        self.window.set_active(true);
        let mut sprite = sfml::graphics::Sprite::with_texture(&self.board_texture);
        sprite.set_scale(texture_scale(self.board_texture.size(), Vector2u::new(8,8)));
        sprite.set_position(BOARD_MARGINS);

        self.window.draw(&sprite);

        self.window.display();
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

fn vector2u_to_vector2f(vec: Vector2u) -> Vector2f {
    let x = vec.x as f32;
    let y = vec.y as f32;
    Vector2f{x, y}
}

fn texture_scale(texture_size: Vector2u, units: Vector2u) -> Vector2f {
    let size = UNIT_SIZE * units;
    let size = vector2u_to_vector2f(size);
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
