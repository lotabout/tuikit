///! A trait defines something that could be drawn
use crate::canvas::Canvas;
use crate::canvas::Result;

/// Something that knows how to draw itself onto the canvas
pub trait Draw {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()>;

    /// the (width, height) of the content
    /// it will be the hint for layouts to calculate the final size
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (None, None)
    }
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        (*self).draw(canvas)
    }

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (*self).size_hint()
    }
}

impl<T: Draw + ?Sized> Draw for Box<T> {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        self.as_ref().draw(canvas)
    }

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        self.as_ref().size_hint()
    }
}
