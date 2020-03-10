//! `Output` is the output stream that deals with ANSI Escape codes.
//! normally you should not use it directly.
//!
//! ```
//! use std::io;
//! use tuikit::attr::Color;
//! use tuikit::output::Output;
//!
//! let mut output = Output::new(Box::new(io::stdout())).unwrap();
//! output.set_fg(Color::YELLOW);
//! output.write("YELLOW\n");
//! output.flush();
//!
//! ```

use std::io;
use std::io::Write;

use crossterm::{cursor, style};
use crossterm::{event, terminal, QueueableCommand};

use crate::attr::{Attr, Color, Effect};

// modeled after python-prompt-toolkit
// term info: https://ftp.netbsd.org/pub/NetBSD/NetBSD-release-7/src/share/terminfo/terminfo

/// Output is an abstraction over the ANSI codes.
pub struct Output {
    /// A callable which returns the `Size` of the output terminal.
    stdout: Box<dyn Write + Send>,
}

impl Output {
    pub fn new(stdout: Box<dyn Write + Send>) -> io::Result<Self> {
        Result::Ok(Self { stdout })
    }

    /// Write text (Terminal escape sequences will be removed/escaped.)
    pub fn write(&mut self, data: &str) {
        let _ = self.stdout.queue(style::Print(data.replace("\x1b", "?")));
    }

    /// Write raw texts to the terminal.
    pub fn write_raw(&mut self, data: &str) {
        let _ = self.stdout.queue(style::Print(data));
    }

    /// Return the encoding for this output, e.g. 'utf-8'.
    /// (This is used mainly to know which characters are supported by the
    /// output the data, so that the UI can provide alternatives, when
    /// required.)
    pub fn encoding(&self) -> &str {
        unimplemented!()
    }

    /// Write to output stream and flush.
    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }

    /// Erases the screen with the background colour and moves the cursor to home.
    pub fn erase_screen(&mut self) {
        let _ = self.stdout.queue(terminal::Clear(terminal::ClearType::All));
    }

    /// Go to the alternate screen buffer. (For full screen applications).
    pub fn enter_alternate_screen(&mut self) {
        let _ = self.stdout.queue(terminal::EnterAlternateScreen);
    }

    /// Leave the alternate screen buffer.
    pub fn quit_alternate_screen(&mut self) {
        let _ = self.stdout.queue(terminal::LeaveAlternateScreen);
    }

    /// Enable mouse.
    pub fn enable_mouse_support(&mut self) {
        let _ = self.stdout.queue(event::EnableMouseCapture);
    }

    /// Disable mouse.
    pub fn disable_mouse_support(&mut self) {
        let _ = self.stdout.queue(event::DisableMouseCapture);
    }

    /// Erases from the current cursor position to the end of the current line.
    pub fn erase_end_of_line(&mut self) {
        let _ = self
            .stdout
            .queue(terminal::Clear(terminal::ClearType::UntilNewLine));
    }

    /// Erases the screen from the current line down to the bottom of the screen.
    pub fn erase_down(&mut self) {
        let _ = self
            .stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown));
    }

    /// Reset color and styling attributes.
    pub fn reset_attributes(&mut self) {
        let _ = self
            .stdout
            .queue(style::SetAttribute(style::Attribute::Reset));
        let _ = self
            .stdout
            .queue(style::SetForegroundColor(style::Color::Reset));
        let _ = self
            .stdout
            .queue(style::SetBackgroundColor(style::Color::Reset));
    }

    /// Set current foreground color
    pub fn set_fg(&mut self, color: Color) {
        match color {
            Color::Default => {
                let _ = self
                    .stdout
                    .queue(style::SetForegroundColor(style::Color::Reset));
            }
            Color::AnsiValue(x) => {
                let _ = self
                    .stdout
                    .queue(style::SetForegroundColor(style::Color::AnsiValue(x)));
            }
            Color::Rgb(r, g, b) => {
                let _ = self
                    .stdout
                    .queue(style::SetForegroundColor(style::Color::Rgb { r, g, b }));
            }
            Color::__Nonexhaustive => unreachable!(),
        }
    }

    /// Set current background color
    pub fn set_bg(&mut self, color: Color) {
        match color {
            Color::Default => {
                let _ = self
                    .stdout
                    .queue(style::SetBackgroundColor(style::Color::Reset));
            }
            Color::AnsiValue(x) => {
                let _ = self
                    .stdout
                    .queue(style::SetBackgroundColor(style::Color::AnsiValue(x)));
            }
            Color::Rgb(r, g, b) => {
                let _ = self
                    .stdout
                    .queue(style::SetBackgroundColor(style::Color::Rgb { r, g, b }));
            }
            Color::__Nonexhaustive => unreachable!(),
        }
    }

    /// Set current effect (underline, bold, etc)
    pub fn set_effect(&mut self, effect: Effect) {
        if effect.contains(Effect::BOLD) {
            let _ = self
                .stdout
                .queue(style::SetAttribute(style::Attribute::Bold));
        }
        if effect.contains(Effect::DIM) {
            let _ = self
                .stdout
                .queue(style::SetAttribute(style::Attribute::Dim));
        }
        if effect.contains(Effect::UNDERLINE) {
            let _ = self
                .stdout
                .queue(style::SetAttribute(style::Attribute::Underlined));
        }
        if effect.contains(Effect::BLINK) {
            let _ = self
                .stdout
                .queue(style::SetAttribute(style::Attribute::SlowBlink));
        }
        if effect.contains(Effect::REVERSE) {
            let _ = self
                .stdout
                .queue(style::SetAttribute(style::Attribute::Reverse));
        }
    }

    /// Set new color and styling attributes.
    pub fn set_attribute(&mut self, attr: Attr) {
        self.set_fg(attr.fg);
        self.set_bg(attr.bg);
        self.set_effect(attr.effect);
    }

    /// Move cursor position.
    pub fn cursor_goto(&mut self, row: usize, column: usize) {
        let _ = self.stdout.queue(cursor::MoveTo(column as u16, row as u16));
    }

    /// Move cursor `amount` place up.
    pub fn cursor_up(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => {
                let _ = self.stdout.queue(cursor::MoveUp(1));
            }
            _ => {
                let _ = self.stdout.queue(cursor::MoveUp(amount as u16));
            }
        }
    }

    /// Move cursor `amount` place down.
    pub fn cursor_down(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => {
                let _ = self.stdout.queue(cursor::MoveDown(1));
            }
            _ => {
                let _ = self.stdout.queue(cursor::MoveDown(amount as u16));
            }
        }
    }

    /// Move cursor `amount` place forward.
    pub fn cursor_forward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => {
                let _ = self.stdout.queue(cursor::MoveRight(1));
            }
            _ => {
                let _ = self.stdout.queue(cursor::MoveRight(amount as u16));
            }
        }
    }

    /// Move cursor `amount` place backward.
    pub fn cursor_backward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => {
                let _ = self.stdout.queue(cursor::MoveLeft(1));
            }
            _ => {
                let _ = self.stdout.queue(cursor::MoveLeft(amount as u16));
            }
        }
    }

    /// Hide cursor.
    pub fn hide_cursor(&mut self) {
        let _ = self.stdout.queue(cursor::Hide);
    }

    /// Show cursor.
    pub fn show_cursor(&mut self) {
        let _ = self.stdout.queue(cursor::Show);
    }

    /// Asks for a cursor position report (CPR). (VT100 only.)
    pub fn ask_for_cpr(&mut self) {
        self.write_raw("\x1b[6n");
        self.flush()
    }

    /// get terminal size (width, height)
    pub fn terminal_size(&self) -> Option<(usize, usize)> {
        terminal::size()
            .map(|(cols, rows)| (cols as usize, rows as usize))
            .ok()
    }

    ///  Execute the command
    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::PutChar(c) => self.write(c.to_string().as_str()),
            Command::Write(content) => self.write(&content),
            Command::Flush => self.flush(),
            Command::EraseScreen => self.erase_screen(),
            Command::AlternateScreen(enable) => {
                if enable {
                    self.enter_alternate_screen()
                } else {
                    self.quit_alternate_screen()
                }
            }
            Command::MouseSupport(enable) => {
                if enable {
                    self.enable_mouse_support();
                } else {
                    self.disable_mouse_support();
                }
            }
            Command::EraseEndOfLine => self.erase_end_of_line(),
            Command::EraseDown => self.erase_down(),
            Command::ResetAttributes => self.reset_attributes(),
            Command::Fg(fg) => self.set_fg(fg),
            Command::Bg(bg) => self.set_bg(bg),
            Command::Effect(effect) => self.set_effect(effect),
            Command::SetAttribute(attr) => self.set_attribute(attr),
            Command::CursorGoto { row, col } => self.cursor_goto(row, col),
            Command::CursorUp(amount) => self.cursor_up(amount),
            Command::CursorDown(amount) => self.cursor_down(amount),
            Command::CursorLeft(amount) => self.cursor_backward(amount),
            Command::CursorRight(amount) => self.cursor_forward(amount),
            Command::CursorShow(show) => {
                if show {
                    self.show_cursor()
                } else {
                    self.hide_cursor()
                }
            }
        }
    }
}

/// Instead of calling functions of `Output`, we could send commands.
#[derive(Debug, Clone)]
pub enum Command {
    /// Put a char to screen
    PutChar(char),
    /// Write content to screen (escape codes will be escaped)
    Write(String),
    /// Flush all the buffered contents
    Flush,
    /// Erase the entire screen
    EraseScreen,
    /// Enter(true)/Quit(false) the alternate screen mode
    AlternateScreen(bool),
    /// Enable(true)/Disable(false) mouse support
    MouseSupport(bool),
    /// Erase contents to the end of current line
    EraseEndOfLine,
    /// Erase contents till the bottom of the screen
    EraseDown,
    /// Reset attributes
    ResetAttributes,
    /// Set the foreground color
    Fg(Color),
    /// Set the background color
    Bg(Color),
    /// Set the effect(e.g. underline, dim, bold, ...)
    Effect(Effect),
    /// Set the fg, bg & effect.
    SetAttribute(Attr),
    /// move the cursor to `(row, col)`
    CursorGoto { row: usize, col: usize },
    /// move cursor up `x` lines
    CursorUp(usize),
    /// move cursor down `x` lines
    CursorDown(usize),
    /// move cursor left `x` characters
    CursorLeft(usize),
    /// move cursor right `x` characters
    CursorRight(usize),
    /// Show(true)/Hide(false) cursor
    CursorShow(bool),
}
