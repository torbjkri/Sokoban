

pub trait Movable {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn undo_last_action(&mut self);
    fn get_last_action(&self) -> Move;
    fn do_action(&mut self, action: Move);
}

#[derive(Copy, Clone)]
pub enum Move {
    None,
    Left,
    Right,
    Up,
    Down
}
