
#[derive(Debug, Clone, Copy)]
pub enum Color {
    AnsiValue(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy)]
pub struct Attrs {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorDepth {
    /// One color only.
    Depth1Bit,
    /// ANSI Colors.
    Depth4Bit,
    /// The default.
    Depth8Bit,
    /// 24 bit True color.
    Depth24Bit,
}

impl ColorDepth {
    // type alias
    pub const MONOCHROME: ColorDepth = ColorDepth::Depth1Bit;
    pub const ANSI_COLORS_ONLY: ColorDepth = ColorDepth::Depth4Bit;
    pub const DEFAULT: ColorDepth = ColorDepth::Depth8Bit;
    pub const TRUE_COLOR: ColorDepth = ColorDepth::Depth24Bit;
}
