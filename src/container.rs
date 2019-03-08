///! Various pre-defined container that implements Draw

use crate::draw::Draw;
use crate::canvas::{Canvas, Result};
use crate::cell::Cell;
use derive_builder::Builder;

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Fixed(usize),
    Percent(usize),
}

///! A box is like a div in HTML, it has its margin/padding, and border
#[derive(Builder, Debug)]
pub struct Box {
    margin_top: Size,
    margin_right: Size,
    margin_bottom: Size,
    margin_left: Size,

    padding_top: Size,
    padding_right: Size,
    padding_bottom: Size,
    padding_left: Size,

    border_top: bool,
    border_right: bool,
    border_bottom: bool,
    border_left: bool,
}

impl Draw for Box {
    fn draw(&self, canvas: &mut Canvas) {
        unimplemented!()
    }
}

pub struct HSplit {

}

impl Draw for HSplit {
    fn draw(&self, canvas: &mut Canvas) {
        unimplemented!()
    }
}

pub struct VSplit {

}

impl Draw for VSplit {
    fn draw(&self, canvas: &mut Canvas) {
        unimplemented!()
    }
}

struct BoundedCanvas<'a, T: Canvas> {
    canvas: &'a mut T,
}

impl<'a, T:Canvas> Canvas for BoundedCanvas<'a, T> {
    fn size(&self) -> Result<(usize, usize)> {
        unimplemented!()
    }

    fn clear(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> Result<()> {
        unimplemented!()
    }

    fn set_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        unimplemented!()
    }

    fn show_cursor(&mut self, show: bool) -> Result<()> {
        unimplemented!()
    }
}
