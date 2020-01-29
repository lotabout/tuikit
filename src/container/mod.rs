///! Various pre-defined container that implements Draw
mod split;
mod win;

pub use self::split::*;
pub use self::win::*;
use crate::draw::Draw;
use std::cmp::min;

/// Whether fixed size or percentage
#[derive(Debug, Copy, Clone)]
pub enum Size {
    Fixed(usize),
    Percent(usize),
    Default,
}

impl Default for Size {
    fn default() -> Self {
        Size::Default
    }
}

impl Size {
    pub fn calc_fixed_size(&self, total_size: usize, default_size: usize) -> usize {
        match *self {
            Size::Fixed(fixed) => min(total_size, fixed),
            Size::Percent(percent) => min(total_size, total_size * percent / 100),
            Size::Default => default_size,
        }
    }
}

impl From<usize> for Size {
    fn from(size: usize) -> Self {
        Size::Fixed(size)
    }
}

/// A container could be recursive nested
pub trait Widget: Draw {
    /// the (width, height) of the content
    /// it will be the hint for layouts to calculate the final size
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (None, None)
    }
}

impl<T: Widget> Widget for &T {
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (*self).size_hint()
    }
}

impl<T: Widget> Widget for Box<T> {
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        self.as_ref().size_hint()
    }
}
