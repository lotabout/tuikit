use bitflags::bitflags;
use std::fmt;

/// `Attribute` is a rendering attribute that contains fg color, bg color and text effect.
///
/// ```
/// use tuikit::attr::{Attr, Color, Effect};
///
/// Attr { fg: Color::RED, effect: Effect::BOLD, ..Attr::default() };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Attr {
    pub fg: Color,
    pub bg: Color,
    pub effect: Effect,
}

impl Default for Attr {
    fn default() -> Self {
        Attr {
            fg: Color::default(),
            bg: Color::default(),
            effect: Effect::empty(),
        }
    }
}

bitflags! {
    pub struct Effect: u8 {
        const BOLD = 0b00000001;
        const DIM = 0b00000010;
        const UNDERLINE = 0b00000100;
        const BLINK = 0b00001000;
        const REVERSE = 0b00010000;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Default,
    AnsiValue(u8),
    Rgb(u8, u8, u8),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Color {
    pub const BLACK: Color = Color::AnsiValue(0);
    pub const RED: Color = Color::AnsiValue(1);
    pub const GREEN: Color = Color::AnsiValue(2);
    pub const YELLOW: Color = Color::AnsiValue(3);
    pub const BLUE: Color = Color::AnsiValue(4);
    pub const MAGENTA: Color = Color::AnsiValue(5);
    pub const CYAN: Color = Color::AnsiValue(6);
    pub const WHITE: Color = Color::AnsiValue(7);
    pub const LIGHT_BLACK: Color = Color::AnsiValue(8);
    pub const LIGHT_RED: Color = Color::AnsiValue(9);
    pub const LIGHT_GREEN: Color = Color::AnsiValue(10);
    pub const LIGHT_YELLOW: Color = Color::AnsiValue(11);
    pub const LIGHT_BLUE: Color = Color::AnsiValue(12);
    pub const LIGHT_MAGENTA: Color = Color::AnsiValue(13);
    pub const LIGHT_CYAN: Color = Color::AnsiValue(14);
    pub const LIGHT_WHITE: Color = Color::AnsiValue(15);
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}
