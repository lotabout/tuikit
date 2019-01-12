use crate::attr::{Attr, Effect};
use crate::color::Color;
use crate::event::Event;
use crate::output::Command;
use std::cmp::min;
use unicode_width::UnicodeWidthChar;

// much of the code comes from https://github.com/agatan/termfest/blob/master/src/screen.rs

/// A Screen is a table of cells to draw on.
/// It's a buffer holding the contents
#[derive(Debug)]
pub struct Screen {
    width: usize,
    height: usize,
    cursor: Cursor,
    cells: Vec<Cell>,

    painted_cells: Vec<Cell>,
    painted_cursor: Cursor,
}

impl Screen {
    /// create an empty screen with size: (width, height)
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            cursor: Cursor {
                row: 0,
                col: 0,
                visible: true,
            },

            painted_cells: vec![Cell::default(); width * height],
            painted_cursor: Cursor {
                row: 0,
                col: 0,
                visible: true,
            },
        }
    }

    /// get the width of the screen
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// get the height of the screen
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(row * self.width + col)
        }
    }

    fn copy_cells(&self, original: &[Cell], width: usize, height: usize) -> Vec<Cell> {
        let mut new_cells = vec![Cell::default(); width * height];
        use std::cmp;
        let min_height = cmp::min(height, self.height);
        let min_width = cmp::min(width, self.width);
        for row in 0..min_height {
            let orig_start = row * self.width;
            let orig_end = min_width + orig_start;
            let start = row * width;
            let end = min_width + start;
            (&mut new_cells[start..end]).copy_from_slice(&original[orig_start..orig_end]);
        }
        new_cells
    }

    /// to resize the screen to `(width, height)`
    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width == width && self.height == height {
            return;
        }

        self.cells = self.copy_cells(&self.cells, width, height);
        self.painted_cells = self.cells.clone();
        self.width = width;
        self.height = height;

        self.cursor.row = min(self.cursor.row, height);
        self.cursor.col = min(self.cursor.col, width);
    }

    /// clear the screen buffer
    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.ch = ' ';
            cell.attr = Attr::default();
        }
    }

    /// sync internal buffer with the terminal
    pub fn present(&mut self) -> Vec<Command> {
        let mut commands = Vec::with_capacity(2048);
        let mut last_cursor = self.painted_cursor;
        let default_attr = Attr::default();
        let mut last_attr = default_attr;

        // hide cursor && reset Attributes
        commands.push(Command::CursorShow(false));
        commands.push(Command::ResetAttributes);

        for row in 0..self.height {
            // calculate the last col that has contents
            let mut empty_col_index = self.width;
            for col in (0..self.width).rev() {
                let index = self.index(row, col).unwrap();
                let cell = &self.cells[index];
                if cell.ch == ' ' && cell.attr == default_attr {
                    self.painted_cells[index] = *cell;
                } else {
                    empty_col_index = col + 1;
                    break;
                }
            }

            // compare cells and print necessary escape codes
            let mut last_ch_is_wide = false;
            for col in 0..empty_col_index {
                let index = self.index(row, col).unwrap();

                // advance if the last character is wide
                if last_ch_is_wide {
                    last_ch_is_wide = false;
                    self.painted_cells[index] = self.cells[index];
                    continue;
                }

                let cell_to_paint = self.cells[index];
                let cell_painted = self.painted_cells[index];

                // no need to paint if the content did not change
                if cell_to_paint == cell_painted {
                    continue;
                }

                // move cursor if necessary
                if last_cursor.row != row || last_cursor.col != col {
                    commands.push(Command::CursorGoto { row, col });
                }

                if cell_to_paint.attr != last_attr {
                    commands.push(Command::ResetAttributes);
                    commands.push(Command::SetAttribute(cell_to_paint.attr));
                    last_attr = cell_to_paint.attr;
                }

                commands.push(Command::PutChar(cell_to_paint.ch));

                let display_width = cell_to_paint.ch.width().unwrap_or(2);

                // wide character
                if display_width == 2 {
                    last_ch_is_wide = true;
                }

                last_cursor.row = row;
                last_cursor.col = col + display_width;
                self.painted_cells[index] = cell_to_paint;
            }

            if empty_col_index != self.width {
                commands.push(Command::CursorGoto {
                    row,
                    col: empty_col_index,
                });
                commands.push(Command::ResetAttributes);
                commands.push(Command::EraseEndOfLine);
                last_attr = Attr::default();
            }
        }

        // restore cursor
        commands.push(Command::CursorGoto {
            row: self.cursor.row,
            col: self.cursor.col,
        });
        if self.cursor.visible {
            commands.push(Command::CursorShow(true));
        }

        self.painted_cursor = self.cursor;

        commands
    }

    /// change a cell of position `(row, col)` to `cell`
    pub fn put_cell(&mut self, row: usize, col: usize, cell: Cell) {
        if let Some(index) = self.index(row, col) {
            self.cells[index] = cell;
        }
    }

    /// print `content` starting with position `(row, col)` with `attr`
    /// - screen will NOT wrap to y+1 if the content is too long
    /// - screen will handle wide characters
    pub fn print(&mut self, row: usize, col: usize, content: &str, attr: Attr) {
        let mut cell = Cell {
            attr,
            ..Cell::default()
        };

        let mut col = col;
        for ch in content.chars() {
            cell.ch = ch;
            self.put_cell(row, col, cell);
            col += ch.width().unwrap_or(2);
        }
    }

    /// set cursor position to (row, col)
    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor.row = min(row, self.height - 1);
        self.cursor.col = min(col, self.width - 1);
    }

    /// show/hide cursor, set `show` to `false` to hide the cursor
    pub fn show_cursor(&mut self, show: bool) {
        self.cursor.visible = show;
    }
}

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

#[derive(Debug, Clone, Copy)]
struct Cursor {
    pub row: usize,
    pub col: usize,
    pub visible: bool,
}
