use raylib::{RaylibHandle};

pub trait Drawable {
    fn draw(&self, rl: &RaylibHandle);
}