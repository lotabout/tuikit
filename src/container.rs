///! Various pre-defined container that implements Draw

use crate::draw::Draw;
use crate::canvas::Canvas;
use crate::cell::Cell;

pub struct Window {

}

impl Draw for Window {
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
    fn size(&self) -> _ {
        unimplemented!()
    }

    fn clear(&mut self) -> _ {
        unimplemented!()
    }

    fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> _ {
        unimplemented!()
    }

    fn set_cursor(&mut self, row: usize, col: usize) -> _ {
        unimplemented!()
    }

    fn show_cursor(&mut self, show: bool) -> _ {
        unimplemented!()
    }
}
