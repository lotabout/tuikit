pub use crate::color::Color;
use bitflags::bitflags;

/// `Attribute` is a rendering attribute that contains fg color, bg color and text effect.
///
/// ```
/// use tuikit::attr::{Attr, Effect, Color};
///
/// Attr { fg: Color::RED, effect: Effect::BOLD, ..Attr::default() };
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
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
