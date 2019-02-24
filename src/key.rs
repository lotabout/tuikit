//! Defines all the keys `tuikit` recognizes.

// http://ascii-table.com/ansi-escape-sequences.php
/// Single key
#[rustfmt::skip]
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Key {
    Null,
    ESC,

    Ctrl(char),
    Tab, // Ctrl-I
    Enter, // Ctrl-M

    BackTab,
    Backspace,

    Del, PgUp, PgDn,

    Up, Down, Left, Right, Home, End, Insert, Delete, PageUp, PageDown,
    CtrlUp, CtrlDown, CtrlLeft, CtrlRight,

    ShiftUp, ShiftDown, ShiftLeft, ShiftRight,

    F(u8),

    AltEnter,
    AltBackspace,

    Alt(char), // chars are lower case
    Char(char), // chars are lower case
    CursorPos(u16, u16), // row, col
    MousePress(MouseButton, u16, u16),
    MouseRelease(u16, u16),
    MouseHold(u16, u16),

    #[doc(hidden)]
    __Nonexhaustive,

}

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with MousePress.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with MousePress.
    WheelDown,
}

#[rustfmt::skip]
pub fn from_keyname(keyname: &str) -> Option<Key> {
    use self::Key::*;
    match keyname.to_lowercase().as_ref() {
        "ctrl-space" | "ctrl-`" | "ctrl-@" => Some(Ctrl(' ')),
        "ctrl-a" => Some(Ctrl('a')),
        "ctrl-b" => Some(Ctrl('b')),
        "ctrl-c" => Some(Ctrl('c')),
        "ctrl-d" => Some(Ctrl('d')),
        "ctrl-e" => Some(Ctrl('e')),
        "ctrl-f" => Some(Ctrl('f')),
        "ctrl-g" => Some(Ctrl('g')),
        "ctrl-h" => Some(Ctrl('h')),
        "tab" | "ctrl-i" => Some(Tab),
        "ctrl-j" => Some(Ctrl('j')),
        "ctrl-k" => Some(Ctrl('k')),
        "ctrl-l" => Some(Ctrl('l')),
        "enter" | "return" | "ctrl-m" => Some(Enter),
        "ctrl-n" => Some(Ctrl('n')),
        "ctrl-o" => Some(Ctrl('o')),
        "ctrl-p" => Some(Ctrl('p')),
        "ctrl-q" => Some(Ctrl('q')),
        "ctrl-r" => Some(Ctrl('r')),
        "ctrl-s" => Some(Ctrl('s')),
        "ctrl-t" => Some(Ctrl('t')),
        "ctrl-u" => Some(Ctrl('u')),
        "ctrl-v" => Some(Ctrl('v')),
        "ctrl-w" => Some(Ctrl('w')),
        "ctrl-x" => Some(Ctrl('x')),
        "ctrl-y" => Some(Ctrl('y')),
        "ctrl-z" => Some(Ctrl('z')),

        "esc"                => Some(ESC),
        "btab" | "shift-tab" => Some(BackTab),
        "bspace" | "bs"      => Some(Backspace),
        "del"                => Some(Delete),
        "pgup" | "page-up"   => Some(PageUp),
        "pgdn" | "page-down" => Some(PageDown),
        "up"                 => Some(Up),
        "down"               => Some(Down),
        "left"               => Some(Left),
        "right"              => Some(Right),
        "home"               => Some(Home),
        "end"                => Some(End),
        "shift-left"         => Some(ShiftLeft),
        "shift-right"        => Some(ShiftRight),

        "f1"  => Some(F(1)),
        "f2"  => Some(F(2)),
        "f3"  => Some(F(3)),
        "f4"  => Some(F(4)),
        "f5"  => Some(F(5)),
        "f6"  => Some(F(6)),
        "f7"  => Some(F(7)),
        "f8"  => Some(F(8)),
        "f9"  => Some(F(9)),
        "f10" => Some(F(10)),
        "f11" => Some(F(11)),
        "f12" => Some(F(12)),

        "altenter"                 => Some(AltEnter),
        "altspace"                 => Some(Alt(' ')),
        "alt-bs" | "alt-backspace" => Some(AltBackspace),

        "alt-a" => Some(Alt('a')),
        "alt-b" => Some(Alt('b')),
        "alt-c" => Some(Alt('c')),
        "alt-d" => Some(Alt('d')),
        "alt-e" => Some(Alt('e')),
        "alt-f" => Some(Alt('f')),
        "alt-g" => Some(Alt('g')),
        "alt-h" => Some(Alt('h')),
        "alt-i" => Some(Alt('i')),
        "alt-j" => Some(Alt('j')),
        "alt-k" => Some(Alt('k')),
        "alt-l" => Some(Alt('l')),
        "alt-m" => Some(Alt('m')),
        "alt-n" => Some(Alt('n')),
        "alt-o" => Some(Alt('o')),
        "alt-p" => Some(Alt('p')),
        "alt-q" => Some(Alt('q')),
        "alt-r" => Some(Alt('r')),
        "alt-s" => Some(Alt('s')),
        "alt-t" => Some(Alt('t')),
        "alt-u" => Some(Alt('u')),
        "alt-v" => Some(Alt('v')),
        "alt-w" => Some(Alt('w')),
        "alt-x" => Some(Alt('x')),
        "alt-y" => Some(Alt('y')),
        "alt-z" => Some(Alt('z')),
        "alt-/" => Some(Alt('/')),

        ch if ch.chars().count() == 1 => {
            Some(Char(ch.chars().next().expect("input:parse_key: no key is specified")))
        },
        _ => None,
    }
}
