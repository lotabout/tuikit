use crate::keys::Key;
use crate::keys::Key::*;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::libc::isatty;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use std::os::unix::io::AsRawFd;
use std::{fs, io};

// taken from termion
/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    unsafe { isatty(stream.as_raw_fd()) == 1 }
}

// taken from termion
/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
}

pub struct KeyBoard {
    file: File,
    buf: VecDeque<char>,
}

// https://www.xfree86.org/4.8.0/ctlseqs.html
impl KeyBoard {
    pub fn new(file: File) -> Self {
        KeyBoard {
            file,
            buf: VecDeque::new(),
        }
    }

    pub fn new_with_tty() -> Self {
        Self::new(get_tty().expect("KeyBoard::new_with_tty: failed to get tty"))
    }

    fn get_chars(&mut self) {
        let mut buf = Vec::with_capacity(10);

        let mut reader_buf = [0; 1];
        let flag = fcntl(self.file.as_raw_fd(), FcntlArg::F_GETFL).expect("Get fcntl failed");
        let mut flag = OFlag::from_bits_truncate(flag);

        // set file to blocking mode and read first byte
        flag.remove(OFlag::O_NONBLOCK);
        let _ = fcntl(self.file.as_raw_fd(), FcntlArg::F_SETFL(flag));
        let _ = self.file.read(&mut reader_buf);
        buf.push(reader_buf[0]);

        // set file to non-blocking mode and read rest bytes (e.g. utf8 or escape codes)
        flag.insert(OFlag::O_NONBLOCK);
        let _ = fcntl(self.file.as_raw_fd(), FcntlArg::F_SETFL(flag));
        while let Ok(_) = self.file.read(&mut reader_buf) {
            buf.push(reader_buf[0]);
        }

        let chars = String::from_utf8(buf).expect("Non UTF8 in input");
        for ch in chars.chars() {
            self.buf.push_back(ch);
        }
    }

    fn next_char(&mut self) -> Result<char, String> {
        if self.buf.is_empty() {
            self.get_chars();
        }
        self.buf
            .pop_front()
            .ok_or("no more bytes in the buffer".to_string())
    }

    pub fn next_key(&mut self) -> Result<Key, Box<dyn Error>> {
        let ch = self.next_char()?;
        match ch {
            '\u{00}' => Ok(Ctrl(' ')),
            '\u{01}' => Ok(Ctrl('A')),
            '\u{02}' => Ok(Ctrl('B')),
            '\u{03}' => Ok(Ctrl('C')),
            '\u{04}' => Ok(Ctrl('D')),
            '\u{05}' => Ok(Ctrl('E')),
            '\u{06}' => Ok(Ctrl('F')),
            '\u{07}' => Ok(Ctrl('G')),
            '\u{08}' => Ok(Ctrl('H')),
            '\u{09}' => Ok(Tab),
            '\u{0A}' => Ok(Ctrl('J')),
            '\u{0B}' => Ok(Ctrl('K')),
            '\u{0C}' => Ok(Ctrl('L')),
            '\u{0D}' => Ok(Enter),
            '\u{0E}' => Ok(Ctrl('N')),
            '\u{0F}' => Ok(Ctrl('O')),
            '\u{10}' => Ok(Ctrl('P')),
            '\u{11}' => Ok(Ctrl('Q')),
            '\u{12}' => Ok(Ctrl('R')),
            '\u{13}' => Ok(Ctrl('S')),
            '\u{14}' => Ok(Ctrl('T')),
            '\u{15}' => Ok(Ctrl('U')),
            '\u{16}' => Ok(Ctrl('V')),
            '\u{17}' => Ok(Ctrl('W')),
            '\u{18}' => Ok(Ctrl('X')),
            '\u{19}' => Ok(Ctrl('Y')),
            '\u{1A}' => Ok(Ctrl('Z')),
            '\u{1B}' => self.escape_sequence(),
            '\u{7F}' => Ok(Backspace),
            ch => Ok(Char(ch)),
        }
    }

    fn escape_sequence(&mut self) -> Result<Key, Box<dyn Error>> {
        let seq1 = self.next_char()?;
        match seq1 {
            '[' => self.escape_csi(),
            'O' => self.escape_o(),
            '\u{1B}' => Ok(ESC), // ESC ESC
            _ => Ok(Alt(seq1)),
        }
    }

    fn escape_csi(&mut self) -> Result<Key, Box<dyn Error>> {
        let seq2 = self.next_char()?;

        let cursor_pos = self.parse_cursor_report();
        if cursor_pos.is_ok() {
            return cursor_pos;
        }

        match seq2 {
            '0' | '9' => Err(format!("unsupported esc sequence: ESC [ {:?}", seq2).into()),
            '1'...'8' => self.extended_escape(seq2),
            '[' => {
                // Linux Console ESC [ [ _
                let seq3 = self.next_char()?;
                match seq3 {
                    'A' => Ok(F(1)),
                    'B' => Ok(F(2)),
                    'C' => Ok(F(3)),
                    'D' => Ok(F(4)),
                    'E' => Ok(F(5)),
                    _ => Err(format!("unsupported esc sequence: ESC [ [ {:?}", seq3).into()),
                }
            }
            'A' => Ok(Up),    // kcuu1
            'B' => Ok(Down),  // kcud1
            'C' => Ok(Right), // kcuf1
            'D' => Ok(Left),  // kcub1
            'H' => Ok(Home),  // khome
            'F' => Ok(End),
            'Z' => Ok(BackTab),
            _ => Err(format!("unsupported esc sequence: ESC [ {:?}", seq2).into()),
        }
    }

    fn parse_cursor_report(&mut self) -> Result<Key, Box<dyn Error>> {
        if self.buf.contains(&';') && self.buf.contains(&'R') {
            let mut row = String::new();
            let mut col = String::new();

            while self.buf.front() != Some(&';') {
                row.push(self.buf.pop_front().unwrap());
            }
            self.buf.pop_front();

            while self.buf.front() != Some(&'R') {
                col.push(self.buf.pop_front().unwrap());
            }
            self.buf.pop_front();

            let row_num = row.parse::<u16>()?;
            let col_num = col.parse::<u16>()?;
            Ok(CursorPos(row_num - 1, col_num - 1))
        } else {
            Err(format!("buffer did not contain cursor position response").into())
        }
    }

    fn extended_escape(&mut self, seq2: char) -> Result<Key, Box<dyn Error>> {
        let seq3 = self.next_char()?;
        if seq3 == '~' {
            match seq2 {
                '1' | '7' => Ok(Home), // tmux, xrvt
                '2' => Ok(Insert),
                '3' => Ok(Delete),    // kdch1
                '4' | '8' => Ok(End), // tmux, xrvt
                '5' => Ok(PageUp),    // kpp
                '6' => Ok(PageDown),  // knp
                _ => Err(format!("unsupported esc sequence: ESC [ {} ~", seq2).into()),
            }
        } else if seq3.is_digit(10) {
            let seq4 = self.next_char()?;
            if seq4 == '~' {
                match (seq2, seq3) {
                    ('1', '1') => Ok(F(1)),  // rxvt-unicode
                    ('1', '2') => Ok(F(2)),  // rxvt-unicode
                    ('1', '3') => Ok(F(3)),  // rxvt-unicode
                    ('1', '4') => Ok(F(4)),  // rxvt-unicode
                    ('1', '5') => Ok(F(5)),  // kf5
                    ('1', '7') => Ok(F(6)),  // kf6
                    ('1', '8') => Ok(F(7)),  // kf7
                    ('1', '9') => Ok(F(8)),  // kf8
                    ('2', '0') => Ok(F(9)),  // kf9
                    ('2', '1') => Ok(F(10)), // kf10
                    ('2', '3') => Ok(F(11)), // kf11
                    ('2', '4') => Ok(F(12)), // kf12
                    _ => Err(format!("unsupported esc sequence: ESC [ {}{} ~", seq2, seq3).into()),
                }
            } else if seq4 == ';' {
                let seq5 = self.next_char()?;
                if seq5.is_digit(10) {
                    let seq6 = self.next_char()?; // '~' expected
                    Err(format!(
                        "unsupported esc sequence: ESC [ {}{} ; {} {}",
                        seq2, seq3, seq5, seq6
                    )
                    .into())
                } else {
                    Err(format!(
                        "unsupported esc sequence: ESC [ {}{} ; {:?}",
                        seq2, seq3, seq5
                    )
                    .into())
                }
            } else {
                Err(format!(
                    "unsupported esc sequence: ESC [ {}{} {:?}",
                    seq2, seq3, seq4
                )
                .into())
            }
        } else if seq3 == ';' {
            let seq4 = self.next_char()?;
            if seq4.is_digit(10) {
                let seq5 = self.next_char()?;
                if seq2 == '1' {
                    match (seq4, seq5) {
                        ('5', 'A') => Ok(CtrlUp),
                        ('5', 'B') => Ok(CtrlDown),
                        ('5', 'C') => Ok(CtrlRight),
                        ('5', 'D') => Ok(CtrlLeft),
                        ('2', 'A') => Ok(ShiftUp),
                        ('2', 'B') => Ok(ShiftDown),
                        ('2', 'C') => Ok(ShiftRight),
                        ('2', 'D') => Ok(ShiftLeft),
                        _ => Err(format!(
                            "unsupported esc sequence: ESC [ 1 ; {} {:?}",
                            seq4, seq5
                        )
                        .into()),
                    }
                } else {
                    Err(format!(
                        "unsupported esc sequence: ESC [ {} ; {} {:?}",
                        seq2, seq4, seq5
                    )
                    .into())
                }
            } else {
                Err(format!("unsupported esc sequence: ESC [ {} ; {:?}", seq2, seq4).into())
            }
        } else {
            match (seq2, seq3) {
                ('5', 'A') => Ok(CtrlUp),
                ('5', 'B') => Ok(CtrlDown),
                ('5', 'C') => Ok(CtrlRight),
                ('5', 'D') => Ok(CtrlLeft),
                _ => Err(format!("unsupported esc sequence: ESC [ {} {:?}", seq2, seq3).into()),
            }
        }
    }

    // SSS3
    fn escape_o(&mut self) -> Result<Key, Box<dyn Error>> {
        let seq2 = self.next_char()?;
        match seq2 {
            'A' => Ok(Up),    // kcuu1
            'B' => Ok(Down),  // kcud1
            'C' => Ok(Right), // kcuf1
            'D' => Ok(Left),  // kcub1
            'F' => Ok(End),   // kend
            'H' => Ok(Home),  // khome
            'P' => Ok(F(1)),  // kf1
            'Q' => Ok(F(2)),  // kf2
            'R' => Ok(F(3)),  // kf3
            'S' => Ok(F(4)),  // kf4
            'a' => Ok(CtrlUp),
            'b' => Ok(CtrlDown),
            'c' => Ok(CtrlRight), // rxvt
            'd' => Ok(CtrlLeft),  // rxvt
            _ => Err(format!("unsupported esc sequence: ESC O {:?}", seq2).into()),
        }
    }
}
