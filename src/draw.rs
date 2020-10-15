///! A trait defines something that could be drawn
use crate::canvas::Canvas;
use crate::Result;

/// Something that knows how to draw itself onto the canvas
pub trait Draw {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()>;
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        (*self).draw(canvas)
    }
}

impl<T: Draw + ?Sized> Draw for Box<T> {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        self.as_ref().draw(canvas)
    }
}
