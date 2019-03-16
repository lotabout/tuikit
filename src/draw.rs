///! A trait defines something that could be drawn
use crate::canvas::Canvas;
use crate::canvas::Result;

/// Something that knows how to draw itself onto the canvas
pub trait Draw {
    fn draw(&self, canvas: &mut Canvas) -> Result<()>;

    /// the (width, height) of the content
    /// will be used by layouts
    fn content_size(&self) -> (usize, usize) {
        (0, 0)
    }
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        (*self).draw(canvas)
    }
}
