///! Various pre-defined container that implements Draw
mod split;
mod win;

pub use self::split::*;
use std::cmp::min;
pub use self::win::*;

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
