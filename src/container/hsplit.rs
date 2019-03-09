use super::Size;
use crate::attr::Attr;
use crate::canvas::{BoundedCanvas, Canvas, Result};
use crate::cell::Cell;
use crate::draw::Draw;
use std::cmp::{max, min};

pub struct Split<'a> {
    basis: Size,
    grow: usize,
    shrink: usize,
    inner: &'a Draw,
}

impl<'a> Split<'a> {
    pub fn new(draw: &'a Draw) -> Self {
        Self {
            basis: Size::Percent(100),
            grow: 1,
            shrink: 1,
            inner: draw,
        }
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

    pub fn get_basis(&self) -> Size {
        self.basis
    }

    pub fn get_grow(&self) -> usize {
        self.grow
    }

    pub fn get_shrink(&self) -> usize {
        self.shrink
    }
}

impl<'a> Draw for Split<'a> {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        self.inner.draw(canvas)
    }
}

pub struct HSplit<'a> {
    splits: Vec<Split<'a>>,
}

impl<'a> Default for HSplit<'a> {
    fn default() -> Self {
        Self { splits: Vec::new() }
    }
}

impl<'a> HSplit<'a> {
    pub fn split(mut self, split: Split<'a>) -> Self {
        self.splits.push(split);
        self
    }
}

enum Op {
    Noop,
    Grow,
    Shrink,
}

impl<'a> Draw for HSplit<'a> {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;

        let split_widths: Vec<usize> = self
            .splits
            .iter()
            .map(Split::get_basis)
            .map(|size| size.calc_fixed_size(width))
            .collect();

        let target_total_width: usize = split_widths.iter().sum();

        let op = if target_total_width == width {
            Op::Noop
        } else if target_total_width < width {
            Op::Grow
        } else {
            Op::Shrink
        };

        let width_diff = match op {
            Op::Noop => 0,
            Op::Grow => width - target_total_width,
            Op::Shrink => target_total_width - width,
        };

        let split_factors: Vec<usize> = self
            .splits
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
            width_diff / total_factors
        };

        // iterate over the splits
        let mut left = 0;
        for (idx, split) in self.splits.iter().enumerate() {
            let diff = min(split_widths[idx], split_factors[idx] * unit);
            let target_width = match op {
                Op::Noop => split_widths[idx],
                Op::Grow => split_widths[idx] + diff,
                Op::Shrink => split_widths[idx] - diff,
            };

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
