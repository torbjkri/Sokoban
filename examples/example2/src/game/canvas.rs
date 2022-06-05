use sfml::graphics::{CircleShape, RenderWindow, RenderTarget, Color, Shape};
use sfml::window::{ContextSettings,Style, VideoMode, Event};

pub struct Canvas<'s> {
    window: RenderWindow,
    circle: CircleShape<'s>,
}

impl<'s> Canvas<'s> {
    pub fn new() -> Self {
        let mut circle = CircleShape::new(100.0, 100);
        circle.set_fill_color(Color::GREEN);
        Canvas{ 
            window: RenderWindow::new(
            VideoMode::new(600, 600, 32),
            "SFML Window",
            Style::CLOSE,
            &ContextSettings::default(),
        ),
        circle: circle}
    }

    pub fn render(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close();
            } 
        }
        self.window.set_active(true);
        self.window.draw(&self.circle);

        self.window.display();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
