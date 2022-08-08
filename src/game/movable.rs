

pub trait Movable {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn undo_last_move(&mut self);
}

pub enum Move {
    None,
    Left,
    Right,
    Up,
    Down
}
