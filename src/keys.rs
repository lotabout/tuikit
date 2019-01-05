// http://ascii-table.com/ansi-escape-sequences.php
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

    Alt(char),
    Char(char),
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
        "ctrl-a" => Some(Ctrl('A')),
        "ctrl-b" => Some(Ctrl('B')),
        "ctrl-c" => Some(Ctrl('C')),
        "ctrl-d" => Some(Ctrl('D')),
        "ctrl-e" => Some(Ctrl('E')),
        "ctrl-f" => Some(Ctrl('F')),
        "ctrl-g" => Some(Ctrl('G')),
        "ctrl-h" => Some(Ctrl('H')),
        "tab" | "ctrl-i" => Some(Tab),
        "ctrl-j" => Some(Ctrl('J')),
        "ctrl-k" => Some(Ctrl('K')),
        "ctrl-l" => Some(Ctrl('L')),
        "enter" | "return" | "ctrl-m" => Some(Enter),
        "ctrl-n" => Some(Ctrl('N')),
        "ctrl-o" => Some(Ctrl('O')),
        "ctrl-p" => Some(Ctrl('P')),
        "ctrl-q" => Some(Ctrl('Q')),
        "ctrl-r" => Some(Ctrl('R')),
        "ctrl-s" => Some(Ctrl('S')),
        "ctrl-t" => Some(Ctrl('T')),
        "ctrl-u" => Some(Ctrl('U')),
        "ctrl-v" => Some(Ctrl('V')),
        "ctrl-w" => Some(Ctrl('W')),
        "ctrl-x" => Some(Ctrl('X')),
        "ctrl-y" => Some(Ctrl('Y')),
        "ctrl-z" => Some(Ctrl('Z')),

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

        "alt-a" => Some(Alt('A')),
        "alt-b" => Some(Alt('B')),
        "alt-c" => Some(Alt('C')),
        "alt-d" => Some(Alt('D')),
        "alt-e" => Some(Alt('E')),
        "alt-f" => Some(Alt('F')),
        "alt-g" => Some(Alt('G')),
        "alt-h" => Some(Alt('H')),
        "alt-i" => Some(Alt('I')),
        "alt-j" => Some(Alt('J')),
        "alt-k" => Some(Alt('K')),
        "alt-l" => Some(Alt('L')),
        "alt-m" => Some(Alt('M')),
        "alt-n" => Some(Alt('N')),
        "alt-o" => Some(Alt('O')),
        "alt-p" => Some(Alt('P')),
        "alt-q" => Some(Alt('Q')),
        "alt-r" => Some(Alt('R')),
        "alt-s" => Some(Alt('S')),
        "alt-t" => Some(Alt('T')),
        "alt-u" => Some(Alt('U')),
        "alt-v" => Some(Alt('V')),
        "alt-w" => Some(Alt('W')),
        "alt-x" => Some(Alt('X')),
        "alt-y" => Some(Alt('Y')),
        "alt-z" => Some(Alt('Z')),
        "alt-/" => Some(Alt('/')),

        ch if ch.chars().count() == 1 => {
            Some(Char(ch.chars().next().expect("input:parse_key: no key is specified")))
        },
        _ => None,
    }
}
