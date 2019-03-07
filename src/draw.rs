///! A trait defines something that could be drawn

use crate::canvas::Canvas;

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas);
}
