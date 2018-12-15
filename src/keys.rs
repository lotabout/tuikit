use std::io::prelude::*;

// http://ascii-table.com/ansi-escape-sequences.php
#[rustfmt::skip]
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Key {
    UnknownEscSeq,
    Null,

    F(u8),

    Up, Down, Left, Right, Home, End, Insert, Delete, PageUp, PageDown,

    ESC,

    BackTab,
    Backspace,

    Char(char),

    Shift(Key),
    Ctrl(Key),
    Tab, // Ctrl('H')
    Enter, // Ctrl('M')
    Alt(Key),

    Pos(u16, u16),
}

#[rustfmt::skip]
pub fn from_keyname(keyname: &str) -> Key {
    use self::Key::*;
    match keyname.to_lowercase().as_ref() {
        "ctrl-space" | "ctrl-`" | "ctrl-@" => Ctrl(Char(' ')),
        "ctrl-a" => Ctrl(Char('A')),
        "ctrl-b" => Ctrl(Char('B')),
        "ctrl-c" => Ctrl(Char('C')),
        "ctrl-d" => Ctrl(Char('D')),
        "ctrl-e" => Ctrl(Char('E')),
        "ctrl-f" => Ctrl(Char('F')),
        "ctrl-g" => Ctrl(Char('G')),
        "ctrl-h" => Ctrl(Char('H')),
        "tab" | "ctrl-i" => Tab,
        "ctrl-j" => Ctrl(Char('J')),
        "ctrl-k" => Ctrl(Char('K')),
        "ctrl-l" => Ctrl(Char('L')),
        "enter" | "return" | "ctrl-m" => Enter,
        "ctrl-n" => Ctrl(Char('N')),
        "ctrl-o" => Ctrl(Char('O')),
        "ctrl-p" => Ctrl(Char('P')),
        "ctrl-q" => Ctrl(Char('Q')),
        "ctrl-r" => Ctrl(Char('R')),
        "ctrl-s" => Ctrl(Char('S')),
        "ctrl-t" => Ctrl(Char('T')),
        "ctrl-u" => Ctrl(Char('U')),
        "ctrl-v" => Ctrl(Char('V')),
        "ctrl-w" => Ctrl(Char('W')),
        "ctrl-x" => Ctrl(Char('X')),
        "ctrl-y" => Ctrl(Char('Y')),
        "ctrl-z" => Ctrl(Char('Z')),

        "esc"                => ESC,
        "btab" | "shift-tab" => BackTab,
        "bspace" | "bs"      => Backspace,
        "del"                => Delete,
        "pgup" | "page-up"   => PageUp,
        "pgdn" | "page-down" => PageDown,
        "up"                 => Up,
        "down"               => Down,
        "left"               => Left,
        "right"              => Right,
        "home"               => Home,
        "end"                => End,
        "shift-left"         => Shift(Left),
        "shift-right"        => Shift(Right),

        "f1"  => F(1),
        "f2"  => F(2),
        "f3"  => F(3),
        "f4"  => F(4),
        "f5"  => F(5),
        "f6"  => F(6),
        "f7"  => F(7),
        "f8"  => F(8),
        "f9"  => F(9),
        "f10" => F(10),
        "f11" => F(11),
        "f12" => F(12),

        "altenter"              => Alt(Enter),
        "altspace"              => Alt(Char(' ')),
        "altslash"              => Alt(Char('\\')),
        "alt-bs" | "alt-bspace" => Alt(Backspace),

        "alt-a" => Alt(Char('A')),
        "alt-b" => Alt(Char('B')),
        "alt-c" => Alt(Char('C')))
        "alt-d" => Alt(Char('D')),
        "alt-e" => Alt(Char('E')),
        "alt-f" => Alt(Char('F')),
        "alt-g" => Alt(Char('G')),
        "alt-h" => Alt(Char('H')),
        "alt-i" => Alt(Char('I')),
        "alt-j" => Alt(Char('J')),
        "alt-k" => Alt(Char('K')),
        "alt-l" => Alt(Char('L')),
        "alt-m" => Alt(Char('M')),
        "alt-n" => Alt(Char('N')),
        "alt-o" => Alt(Char('O')),
        "alt-p" => Alt(Char('P')),
        "alt-q" => Alt(Char('Q')),
        "alt-r" => Alt(Char('R')),
        "alt-s" => Alt(Char('S')),
        "alt-t" => Alt(Char('T')),
        "alt-u" => Alt(Char('U')),
        "alt-v" => Alt(Char('V')),
        "alt-w" => Alt(Char('W')),
        "alt-x" => Alt(Char('X')),
        "alt-y" => Alt(Char('Y')),
        "alt-z" => Alt(Char('Z')),

        ch if ch.chars().count() == 1 => {
            Key::Char(ch.chars().next().expect("input:parse_key: no key is specified"))
        },
        _ => None,
    }
}

pub fn from_char(c: char) -> KeyPress {
    use self::Key::*;
    if !c.is_control() {
        return Char(c);
    }

    #[allow(clippy::match_same_arms)]
    match c {
        '\x00' => Ctrl(Char(' ')),
        '\x01' => Ctrl(Char('A')),
        '\x02' => Ctrl(Char('B')),
        '\x03' => Ctrl(Char('C')),
        '\x04' => Ctrl(Char('D')),
        '\x05' => Ctrl(Char('E')),
        '\x06' => Ctrl(Char('F')),
        '\x07' => Ctrl(Char('G')),
        '\x08' => Backspace, // '\b'
        '\x09' => Tab,       // '\t'
        '\x0a' => Ctrl(Char('J')), // '\n' (10)
        '\x0b' => Ctrl(Char('K')),
        '\x0c' => Ctrl(Char('L')),
        '\x0d' => Enter, // '\r' (13)
        '\x0e' => Ctrl(Char('N')),
        '\x0f' => Ctrl(Char('O')),
        '\x10' => Ctrl(Char('P')),
        '\x12' => Ctrl(Char('R')),
        '\x13' => Ctrl(Char('S')),
        '\x14' => Ctrl(Char('T')),
        '\x15' => Ctrl(Char('U')),
        '\x16' => Ctrl(Char('V')),
        '\x17' => Ctrl(Char('W')),
        '\x18' => Ctrl(Char('X')),
        '\x19' => Ctrl(Char('Y')),
        '\x1a' => Ctrl(Char('Z')),
        '\x1b' => Esc, // Ctrl-[
        '\x1c' => Ctrl(Char('\\')),
        '\x1d' => Ctrl(Char(']')),
        '\x1e' => Ctrl(Char('^')),
        '\x1f' => Ctrl(Char('_')),
        '\x7f' => Backspace, // Rubout
        _ => Null,
    }
}
