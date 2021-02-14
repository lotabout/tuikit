///! A trait defines something that could be drawn
use crate::canvas::Canvas;

pub type DrawResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Something that knows how to draw itself onto the canvas
pub trait Draw {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()>;
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        (*self).draw(canvas)
    }
}

impl<T: Draw + ?Sized> Draw for Box<T> {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        self.as_ref().draw(canvas)
    }
}
