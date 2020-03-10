//! module to handle keystrokes
//!
//! ```no_run
//! use tuikit::input::KeyBoard;
//! use tuikit::key::Key;
//! use std::time::Duration;
//! let mut keyboard = KeyBoard::new_with_tty();
//! let key = keyboard.next_key();
//! ```

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEvent};
use nix::fcntl::{fcntl, FcntlArg, OFlag};

use crate::event::Event::Resize;
use crate::key::{Key, MouseButton};
use crate::key::Key::*;
use crate::raw::get_tty;
use crate::spinlock::SpinLock;
use crate::sys::file::wait_until_ready;

pub struct KeyBoard {}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// https://www.xfree86.org/4.8.0/ctlseqs.html
// http://man7.org/linux/man-pages/man4/console_codes.4.html
impl KeyBoard {
    pub fn new_with_tty() -> Self {
        Self {}
    }

    /// Wait next key stroke
    pub fn next_key(&mut self) -> Result<Key> {
        crossterm::event::read()
            .map_err(|err| err.to_string().into())
            .and_then(KeyBoard::parse_event)
    }

    /// Wait `timeout` until next key stroke
    pub fn next_key_timeout(&mut self, timeout: Duration) -> Result<Key> {
        if crossterm::event::poll(timeout)? {
            self.next_key()
        } else {
            Err("timeout waiting for new key".into())
        }
    }

    fn parse_event(event: Event) -> Result<Key> {
        Ok(match event {
            Event::Key(key) => match key.code {
                KeyCode::Backspace => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltBackspace
                    } else {
                        Backspace
                    }
                }
                KeyCode::Enter => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltEnter
                    } else {
                        Enter
                    }
                }
                KeyCode::Left => {
                    if key.modifiers.contains(KeyModifiers::ALT)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        AltShiftLeft
                    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                        CtrlLeft
                    } else if key.modifiers.contains(KeyModifiers::ALT) {
                        AltLeft
                    } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                        ShiftLeft
                    } else {
                        Left
                    }
                }
                KeyCode::Right => {
                    if key.modifiers.contains(KeyModifiers::ALT)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        AltShiftRight
                    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                        CtrlRight
                    } else if key.modifiers.contains(KeyModifiers::ALT) {
                        AltRight
                    } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                        ShiftRight
                    } else {
                        Right
                    }
                }
                KeyCode::Up => {
                    if key.modifiers.contains(KeyModifiers::ALT)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        AltShiftUp
                    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                        CtrlUp
                    } else if key.modifiers.contains(KeyModifiers::ALT) {
                        AltUp
                    } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                        ShiftUp
                    } else {
                        Up
                    }
                }
                KeyCode::Down => {
                    if key.modifiers.contains(KeyModifiers::ALT)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        AltShiftDown
                    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                        CtrlDown
                    } else if key.modifiers.contains(KeyModifiers::ALT) {
                        AltDown
                    } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                        ShiftDown
                    } else {
                        Down
                    }
                }
                KeyCode::Home => Home,
                KeyCode::End => End,
                KeyCode::PageUp => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltPageUp
                    } else {
                        PageUp
                    }
                }
                KeyCode::PageDown => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltPageUp
                    } else {
                        PageUp
                    }
                }
                KeyCode::Tab => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltTab
                    } else {
                        Tab
                    }
                }
                KeyCode::BackTab => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        AltBackTab
                    } else {
                        BackTab
                    }
                }
                KeyCode::Delete => Delete,
                KeyCode::Insert => Insert,
                KeyCode::F(key) => F(key),
                KeyCode::Char(ch) => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        Alt(ch)
                    } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                        Ctrl(ch)
                    } else {
                        Char(ch)
                    }
                }
                KeyCode::Null => Null,
                KeyCode::Esc => ESC,
            },
            Event::Mouse(mouse) => match mouse {
                MouseEvent::Down(btn, col, row, modifier) => match btn {
                    event::MouseButton::Left => MousePress(MouseButton::Left, row, col),
                    event::MouseButton::Right => MousePress(MouseButton::Right, row, col),
                    event::MouseButton::Middle => MousePress(MouseButton::Middle, row, col),
                },
                MouseEvent::Up(btn, col, row, modifier) => match btn {
                    event::MouseButton::Left => MouseRelease(row, col),
                    event::MouseButton::Right => MouseRelease(row, col),
                    event::MouseButton::Middle => MouseRelease(row, col),
                },
                MouseEvent::Drag(btn, col, row, modifier) => match btn {
                    event::MouseButton::Left => MouseHold(row, col),
                    event::MouseButton::Right => MouseHold(row, col),
                    event::MouseButton::Middle => MouseHold(row, col),
                },
                MouseEvent::ScrollDown(col, row, modifier) => MousePress(MouseButton::WheelDown, row, col),
                MouseEvent::ScrollUp(col, row, modifier) => MousePress(MouseButton::WheelUp, row, col),
            },
            Event::Resize(cols, rows) => Resize {
                width: rows as usize,
                height: cols as usize,
            },
        })
    }
}
