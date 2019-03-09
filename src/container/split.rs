use super::Size;
use crate::attr::Attr;
use crate::canvas::{BoundedCanvas, Canvas, Result};
use crate::cell::Cell;
use crate::draw::Draw;
use std::cmp::{max, min};

pub trait Split: Draw {
    fn get_basis(&self) -> Size;

    fn get_grow(&self) -> usize;

    fn get_shrink(&self) -> usize;
}

impl<T: Split + Draw> Split for &T {
    fn get_basis(&self) -> Size {
        (*self).get_basis()
    }

    fn get_grow(&self) -> usize {
        (*self).get_grow()
    }

    fn get_shrink(&self) -> usize {
        (*self).get_shrink()
    }
}

enum Op {
    Noop,
    Grow,
    Shrink,
}

trait SplitContainer<'a> {
    fn get_splits(&self) -> &[Box<Split + 'a>];

    /// return the target sizes of the splits
    fn retrieve_split_info(&self, actual_size: usize) -> Vec<usize> {
        let split_sizes: Vec<usize> = self
            .get_splits()
            .iter()
            .map(|split| split.get_basis())
            .map(|size| size.calc_fixed_size(actual_size))
            .collect();

        let target_total_size: usize = split_sizes.iter().sum();

        let op = if target_total_size == actual_size {
            Op::Noop
        } else if target_total_size < actual_size {
            Op::Grow
        } else {
            Op::Shrink
        };

        let size_diff = match op {
            Op::Noop => 0,
            Op::Grow => actual_size - target_total_size,
            Op::Shrink => target_total_size - actual_size,
        };

        let split_factors: Vec<usize> = self
            .get_splits()
            .iter()
            .map(|split| match op {
                Op::Noop => 0,
                Op::Shrink => split.get_shrink(),
                Op::Grow => split.get_grow(),
            })
            .collect();

        let total_factors: usize = split_factors.iter().sum();

        let unit = if total_factors == 0 {
            0
        } else {
            size_diff / total_factors
        };

        self.get_splits()
            .iter()
            .enumerate()
            .map(|(idx, split)| {
                let diff = split_factors[idx] * unit;
                match op {
                    Op::Noop => split_sizes[idx],
                    Op::Grow => split_sizes[idx] + diff,
                    Op::Shrink => split_sizes[idx] - min(split_sizes[idx], diff),
                }
            })
            .collect()
    }

}

pub struct HSplit<'a> {
    basis: Size,
    grow: usize,
    shrink: usize,
    splits: Vec<Box<Split + 'a>>,
}

impl<'a> Default for HSplit<'a> {
    fn default() -> Self {
        Self {
            basis: Size::Percent(100),
            grow: 1,
            shrink: 1,
            splits: Vec::new()
        }
    }
}

impl<'a> HSplit<'a> {
    pub fn split(mut self, split: impl Split + 'a) -> Self {
        self.splits.push(Box::new(split));
        self
    }

    pub fn basis(mut self, basis: Size) -> Self {
        self.basis = basis;
        self
    }

    pub fn grow(mut self, grow: usize) -> Self {
        self.grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: usize) -> Self {
        self.shrink = shrink;
        self
    }
}

impl<'a> SplitContainer<'a> for HSplit<'a> {

    fn get_splits(&self) -> &[Box<Split + 'a>] {
        &self.splits
    }
}

impl<'a> Draw for HSplit<'a> {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let target_widths = self.retrieve_split_info(width);

        // iterate over the splits
        let mut left = 0;
        for (idx, split) in self.splits.iter().enumerate() {
            let target_width = target_widths[idx];
            let right = min(left + target_width, width);
            let mut new_canvas = BoundedCanvas::new(0, left, right - left, height, canvas);
            split.draw(&mut new_canvas);

            if right >= width {
                break;
            }
            left = right;
        }

        Ok(())
    }
}

impl<'a> Split for HSplit<'a> {
    fn get_basis(&self) -> Size {
        self.basis
    }

    fn get_grow(&self) -> usize {
        self.grow
    }

    fn get_shrink(&self) -> usize {
        self.shrink
    }
}





pub struct VSplit<'a> {
    basis: Size,
    grow: usize,
    shrink: usize,
    splits: Vec<Box<Split + 'a>>,
}

impl<'a> Default for VSplit<'a> {
    fn default() -> Self {
        Self {
            basis: Size::Percent(100),
            grow: 1,
            shrink: 1,
            splits: Vec::new()
        }
    }
}

impl<'a> VSplit<'a> {
    pub fn split(mut self, split: impl Split + 'a) -> Self {
        self.splits.push(Box::new(split));
        self
    }

    pub fn basis(mut self, basis: Size) -> Self {
        self.basis = basis;
        self
    }

    pub fn grow(mut self, grow: usize) -> Self {
        self.grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: usize) -> Self {
        self.shrink = shrink;
        self
    }
}

impl<'a> SplitContainer<'a> for VSplit<'a> {

    fn get_splits(&self) -> &[Box<Split + 'a>] {
        &self.splits
    }
}

impl<'a> Draw for VSplit<'a> {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let target_heights = self.retrieve_split_info(height);

        // iterate over the splits
        let mut top = 0;
        for (idx, split) in self.splits.iter().enumerate() {
            let target_height = target_heights[idx];
            let bottom = min(top + target_height, height);
            let mut new_canvas = BoundedCanvas::new(top, 0, width, bottom - top, canvas);
            split.draw(&mut new_canvas);

            if bottom >= height {
                break;
            }
            top = bottom;
        }

        Ok(())
    }
}

impl<'a> Split for VSplit<'a> {
    fn get_basis(&self) -> Size {
        self.basis
    }

    fn get_grow(&self) -> usize {
        self.grow
    }

    fn get_shrink(&self) -> usize {
        self.shrink
    }
}
