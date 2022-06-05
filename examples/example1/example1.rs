use sfml::graphics::{CircleShape, RenderTarget, RenderWindow, Color, Shape};
use sfml::window::{ContextSettings, Event, Style, VideoMode};

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(200, 200, 32),
        "SFML Window",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);

    let mut circle = CircleShape::new(100.0, 100);

    circle.set_fill_color(Color::GREEN);

}
