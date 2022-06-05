pub struct Application<'a> {
    canvas: super::canvas::Canvas<'a>
}
impl<'a> Application<'a> {
    pub fn new() -> Self {
        Application{
            canvas: super::canvas::Canvas::new()
        }
    }

    pub fn run(&mut self) {
        while self.canvas.is_open() {
            self.canvas.render();
        }
    }
}