use crate::attr::{Attr, Effect};
use crate::color::Color;
use crate::event::Event;

// modeled after termbox

/// `Cell` is a cell of the terminal.
/// It has a display character and an attribute (fg and bg color, effects).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub attr: Attr,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            attr: Attr::default(),
        }
    }
}

impl Cell {
    pub fn ch(mut self, ch: char) -> Self {
        self.ch = ch;
        self
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.attr.fg = fg;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.attr.bg = bg;
        self
    }

    pub fn effect(mut self, effect: Effect) -> Self {
        self.attr.effect = effect;
        self
    }

    pub fn attribute(mut self, attr: Attr) -> Self {
        self.attr = attr;
        self
    }
}

/// A Screen is is an abstraction over terminal, viewing it as a table of fixed-size cells
/// and input as a stream of Events.
pub trait Screen {
    /// get the width of the screen
    fn width(&self) -> usize;

    /// get the height of the screen
    fn height(&self) -> usize;

    /// clear buffer
    fn clear(&mut self);

    /// sync internal buffer with the terminal
    fn present(&mut self);

    /// change a cell of position `(x, y)` or say `(row, col)` to `cell`
    fn change_cell(&mut self, x: usize, y: usize, cell: Cell);

    /// print `content` starting with position `(x, y)` with `attr`
    /// - screen will NOT wrap to y+1 if the content is too long
    /// - screen will handle wide characters
    fn print(&mut self, x: usize, y: usize, content: &str, attr: Attr);

    /// set cursor position to (x, y) or say (row, col)
    fn set_cursor(&mut self, x: usize, y: usize);

    /// show/hide cursor, set `show` to `false` to hide the cursor
    fn show_cursor(&mut self, show: bool);

    /// wait an event up to timeout_mills milliseconds and return it
    fn peek_event(&mut self, timeout_mills: u32) -> Option<Event>;

    /// wait for an event and return it
    fn poll_event(&mut self) -> Option<Event>;
}
