use crate::attr::Attr;
///! A canvas is a trait for defining the draw actions
use crate::cell::Cell;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Canvas {
    /// Get the canvas size (width, height)
    fn size(&self) -> Result<(usize, usize)>;

    /// clear the canvas
    fn clear(&mut self) -> Result<()>;

    /// change a cell of position `(row, col)` to `cell`
    fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> Result<()>;

    /// print `content` starting with position `(row, col)` with `attr`
    fn print_with_attr(&mut self, row: usize, col: usize, content: &str, attr: Attr) -> Result<()>;

    /// move cursor position (row, col) and show cursor
    fn set_cursor(&mut self, row: usize, col: usize) -> Result<()>;

    /// show/hide cursor, set `show` to `false` to hide the cursor
    fn show_cursor(&mut self, show: bool) -> Result<()>;
}
