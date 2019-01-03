use std::io;
use std::io::{Stdout, Write};
use std::os::unix::io::AsRawFd;

use crate::attr::Attrs;
use crate::attr::ColorDepth;
use crate::sys::size::terminal_size;

use term::terminfo::TermInfo;

// modeled after python-prompt-toolkit
// term info: https://ftp.netbsd.org/pub/NetBSD/NetBSD-release-7/src/share/terminfo/terminfo

const DEFAULT_BUFFER_SIZE: usize = 1024;

pub struct Output {
    /// A callable which returns the `Size` of the output terminal.
    buffer: Vec<u8>,
    stdout: Stdout,
    /// The terminal environment variable. (xterm, xterm-256color, linux, ...)
    terminfo: TermInfo,
}

/// Output is an abstraction over the ANSI codes.
impl Output {
    pub fn new(stdout: Stdout) -> io::Result<Self> {
        Result::Ok(Self {
            buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
            stdout,
            terminfo: TermInfo::from_env()?,
        })
    }

    fn write_if_exists(&mut self, typ: &str) {
        if let Some(bytes) = self.terminfo.strings.get(typ) {
            self.buffer.extend(bytes)
        }
    }

    /// Write text (Terminal escape sequences will be removed/escaped.)
    pub fn write(&mut self, data: &str) {
        self.buffer.extend(data.replace("0x1b", "?").as_bytes());
    }

    /// Write text.
    pub fn write_raw(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Return the encoding for this output, e.g. 'utf-8'.
    /// (This is used mainly to know which characters are supported by the
    /// output the data, so that the UI can provide alternatives, when
    /// required.)
    pub fn encoding(&self) -> &str {
        unimplemented!()
    }

    /// Set terminal title.
    pub fn set_title(&mut self, title: &str) {
        if self.terminfo.names.contains(&"linux".to_string())
            || self.terminfo.names.contains(&"eterm-color".to_string())
        {
            return;
        }

        let title = title.replace("\x1b", "").replace("\x07", "");
        self.write_raw(format!("\x1b]2;{}\x07", title).as_bytes());
    }

    /// Clear title again. (or restore previous title.)
    pub fn clear_title(&mut self) {
        self.set_title("");
    }

    /// Write to output stream and flush.
    pub fn flush(&mut self) {
        let mut stdout = self.stdout.lock();
        let _ = stdout.write(&self.buffer);
        self.buffer.clear();
        let _ = stdout.flush();
    }

    /// Erases the screen with the background colour and moves the cursor to home.
    pub fn erase_screen(&mut self) {
        self.write_if_exists("clear");
    }

    /// Go to the alternate screen buffer. (For full screen applications).
    pub fn enter_alternate_screen(&mut self) {
        self.write_if_exists("smcup");
    }

    /// Leave the alternate screen buffer.
    pub fn quit_alternate_screen(&mut self) {
        self.write_if_exists("rmcup");
    }

    /// Enable mouse.
    pub fn enable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000h".as_bytes());

        // Enable urxvt Mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1015h".as_bytes());

        // Also enable Xterm SGR mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1006h".as_bytes());

        // Note: E.g. lxterminal understands 1000h, but not the urxvt or sgr extensions.
    }

    /// Disable mouse.
    pub fn disable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000l".as_bytes());
        self.write_raw("\x1b[?1015l".as_bytes());
        self.write_raw("\x1b[?1006l".as_bytes());
    }

    /// Erases from the current cursor position to the end of the current line.
    pub fn erase_end_of_line(&mut self) {
        self.write_if_exists("el");
    }

    /// Erases the screen from the current line down to the bottom of the screen.
    pub fn erase_down(&mut self) {
        self.write_if_exists("ed");
    }

    /// Reset color and styling attributes.
    pub fn reset_attributes(&mut self) {
        self.write_raw("\x1b[0m".as_bytes());
    }

    /// Set new color and styling attributes.
    pub fn set_attributes(&mut self, attrs: Attrs, color_depth: ColorDepth) {
        unimplemented!()
    }

    /// Disable auto line wrapping.
    pub fn disable_autowrap(&mut self) {
        self.write_if_exists("rmam");
    }

    /// Enable auto line wrapping.
    pub fn enable_autowrap(&mut self) {
        self.write_if_exists("smam");
    }

    /// Move cursor position.
    pub fn cursor_goto(&mut self, row: usize, column: usize) {
        self.write_raw(format!("\x1b[{};{}H", row, column).as_bytes());
    }

    /// Move cursor `amount` place up.
    pub fn cursor_up(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_if_exists("kcuu1"),
            _ => self.write_raw(format!("\x1b[{}A", amount).as_bytes()),
        }
    }

    /// Move cursor `amount` place down.
    pub fn cursor_down(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_if_exists("kcud1"),
            _ => self.write_raw(format!("\x1b[{}B", amount).as_bytes()),
        }
    }

    /// Move cursor `amount` place forward.
    pub fn cursor_forward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_if_exists("kcuf1"),
            _ => self.write_raw(format!("\x1b[{}C", amount).as_bytes()),
        }
    }

    /// Move cursor `amount` place backward.
    pub fn cursor_backward(&mut self, amount: usize) {
        match amount {
            0 => {}
            1 => self.write_if_exists("kcub1"),
            _ => self.write_raw(format!("\x1b[{}D", amount).as_bytes()),
        }
    }

    /// Hide cursor.
    pub fn hide_cursor(&mut self) {
        self.write_if_exists("civis");
    }

    /// Show cursor.
    pub fn show_cursor(&mut self) {
        self.write_if_exists("cnorm");
    }

    /// Asks for a cursor position report (CPR). (VT100 only.)
    pub fn ask_for_cpr(&mut self) {
        self.write_if_exists("u7");
        self.flush()
    }

    /// Sound bell.
    pub fn bell(&mut self) {
        self.write_if_exists("bel"); // \a
        self.flush()
    }

    /// get terminal size (width, height)
    pub fn terminal_size(&self) -> io::Result<(u16, u16)> {
        terminal_size(self.stdout.as_raw_fd())
    }

    /// For vt100 only. "
    pub fn enable_bracketed_paste(&mut self) {
        self.write_raw("\x1b[?2004h".as_bytes());
    }

    /// For vt100 only.
    pub fn disable_bracketed_paste(&mut self) {
        self.write_raw("\x1b[?2004l".as_bytes());
    }
}
