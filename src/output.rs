use std::io;
use std::io::{Stdout, Write};
use std::os::unix::io::AsRawFd;

use crate::attr::Attrs;
use crate::attr::ColorDepth;
use crate::sys::size::terminal_size;

// modeled after python-prompt-toolkit

const DEFAULT_BUFFER_SIZE: usize = 1024;

pub struct Output {
    /// A callable which returns the `Size` of the output terminal.
    buffer: String,
    stdout: Stdout,
    /// The terminal environment variable. (xterm, xterm-256color, linux, ...)
    term: String,
}

/// Output is an abstraction over the ANSI codes.
impl Output {
    pub fn new(stdout: Stdout, term: Option<&str>) -> Self {
        Self {
            buffer: String::with_capacity(DEFAULT_BUFFER_SIZE),
            stdout,
            term: term.unwrap_or("xterm").to_string(),
        }
    }

    /// Return the encoding for this output, e.g. 'utf-8'.
    /// (This is used mainly to know which characters are supported by the
    /// output the data, so that the UI can provide alternatives, when
    /// required.)
    pub fn encoding(&self) -> &str {
        unimplemented!()
    }

    /// Write text (Terminal escape sequences will be removed/escaped.)
    pub fn write(&mut self, data: &str) {
        self.buffer.push_str(&data.replace("0x1b", "?"));
    }

    /// Write text.
    pub fn write_raw(&mut self, data: &str) {
        self.buffer.push_str(data);
    }

    /// Set terminal title.
    pub fn set_title(&mut self, title: &str) {
        match title {
            "linux" | "eterm-color" => {
                // title not supported
            }
            _ => {
                let title = title.replace("\x1b", "").replace("\x07", "");
                self.write_raw(format!("\x1b]2;{}\x07", title).as_str());
            }
        }
    }

    /// Clear title again. (or restore previous title.)
    pub fn clear_title(&mut self) {
        self.set_title("");
    }

    /// Write to output stream and flush.
    pub fn flush(&mut self) {
        let mut stdout = self.stdout.lock();
        let _ = stdout.write(self.buffer.as_bytes());
        self.buffer.clear();
        let _ = stdout.flush();
    }

    /// Erases the screen with the background colour and moves the cursor to home.
    pub fn erase_screen(&mut self) {
        self.write_raw("\x1b[2J");
    }

    /// Go to the alternate screen buffer. (For full screen applications).
    pub fn enter_alternate_screen(&mut self) {
        self.write_raw("\x1b[?1049h\x1b[H");
    }

    /// Leave the alternate screen buffer.
    pub fn quit_alternate_screen(&mut self) {
        self.write_raw("\x1b[?1049l");
    }

    /// Enable mouse.
    pub fn enable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000h");

        // Enable urxvt Mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1015h");

        // Also enable Xterm SGR mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1006h");

        // Note: E.g. lxterminal understands 1000h, but not the urxvt or sgr extensions.
    }

    /// Disable mouse.
    pub fn disable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000l");
        self.write_raw("\x1b[?1015l");
        self.write_raw("\x1b[?1006l");
    }

    /// Erases from the current cursor position to the end of the current line.
    pub fn erase_end_of_line(&mut self) {
        self.write_raw("\x1b[K");
    }

    /// Erases the screen from the current line down to the bottom of the screen.
    pub fn erase_down(&mut self) {
        self.write_raw("\x1b[J");
    }

    /// Reset color and styling attributes.
    pub fn reset_attributes(&mut self) {
        self.write_raw("\x1b[0m");
    }

    /// Set new color and styling attributes.
    pub fn set_attributes(&mut self, attrs: Attrs, color_depth: ColorDepth) {
        unimplemented!()
    }

    /// Disable auto line wrapping.
    pub fn disable_autowrap(&mut self) {
        self.write_raw("\x1b[?7l");
    }

    /// Enable auto line wrapping.
    pub fn enable_autowrap(&mut self) {
        self.write_raw("\x1b[?7h");
    }

    /// Move cursor position.
    pub fn cursor_goto(&mut self, row: usize, column: usize) {
        self.write_raw(format!("\x1b[{};{}H", row, column).as_str());
    }

    /// Move cursor `amount` place up.
    pub fn cursor_up(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_raw("\x1b[A"),
            _ => self.write_raw(format!("\x1b[{}A", amount).as_str()),
        }
    }

    /// Move cursor `amount` place down.
    pub fn cursor_down(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_raw("\x1b[B"),
            _ => self.write_raw(format!("\x1b[{}B", amount).as_str()),
        }
    }

    /// Move cursor `amount` place forward.
    pub fn cursor_forward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_raw("\x1b[C"),
            _ => self.write_raw(format!("\x1b[{}C", amount).as_str()),
        }
    }

    /// Move cursor `amount` place backward.
    pub fn cursor_backward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_raw("\x1b[D"),
            _ => self.write_raw(format!("\x1b[{}D", amount).as_str()),
        }
    }

    /// Hide cursor.
    pub fn hide_cursor(&mut self) {
        self.write_raw("\x1b[?25l");
    }

    /// Show cursor.
    pub fn show_cursor(&mut self) {
        self.write_raw("\x1b[?12l\x1b[?25h"); // Stop blinking cursor and show.
    }

    /// Asks for a cursor position report (CPR). (VT100 only.)
    pub fn ask_for_cpr(&mut self) {
        self.write_raw("\x1b[6n");
        self.flush()
    }

    /// Sound bell.
    pub fn bell(&mut self) {
        self.write_raw("\x07"); // \a
        self.flush()
    }

    /// get terminal size (width, height)
    pub fn terminal_size(&self) -> io::Result<(u16, u16)> {
        terminal_size(self.stdout.as_raw_fd())
    }

    /// For vt100 only. "
    pub fn enable_bracketed_paste(&mut self) {
        self.write_raw("\x1b[?2004h");
    }

    /// For vt100 only.
    pub fn disable_bracketed_paste(&mut self) {
        self.write_raw("\x1b[?2004l");
    }
}
