///! A trait defines something that could be drawn
use crate::canvas::Canvas;
use crate::canvas::Result;

/// Something that knows how to draw itself onto the canvas
pub trait Draw {
    fn draw(&self, canvas: &mut Canvas) -> Result<()>;

    /// the (width, height) of the content
    /// it will be the hint for layouts to calculate the final size
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (None, None)
    }
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        (*self).draw(canvas)
    }

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (*self).size_hint()
    }
}
