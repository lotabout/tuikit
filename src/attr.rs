//! attr modules defines the attributes(colors, effects) of a terminal cell

pub use crate::color::Color;
use bitflags::bitflags;

/// `Attr` is a rendering attribute that contains fg color, bg color and text effect.
///
/// ```
/// use tuikit::attr::{Attr, Effect, Color};
///
/// let attr = Attr { fg: Color::RED, effect: Effect::BOLD, ..Attr::default() };
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
    /// `Effect` is the effect of a text
    pub struct Effect: u8 {
        const BOLD = 0b00000001;
        const DIM = 0b00000010;
        const UNDERLINE = 0b00000100;
        const BLINK = 0b00001000;
        const REVERSE = 0b00010000;
    }
}
