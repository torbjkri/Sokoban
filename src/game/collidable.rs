

pub trait Collidable {
    fn check_collision(&self, other: &dyn Collidable) -> bool;
}
