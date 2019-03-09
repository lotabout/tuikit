///! Various pre-defined container that implements Draw
mod win;
use std::cmp::min;
pub use win::*;

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Fixed(usize),
    Percent(usize),
}

impl Default for Size {
    fn default() -> Self {
        Size::Fixed(0)
    }
}

impl Size {
    pub fn calc_fixed_size(&self, total_size: usize) -> usize {
        match *self {
            Size::Fixed(fixed) => min(total_size, fixed),
            Size::Percent(percent) => min(total_size, total_size * percent / 100),
        }
    }
}
