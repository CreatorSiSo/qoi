/// 0x716f6966
pub const MAGIC: [u8; 4] = *b"qoif";
pub const HEADER_SIZE: usize = 14;
pub const PADDING: usize = 4;
pub const _END_MARKER_SIZE: usize = 8;

/// 0b00000000
pub const _OP_INDEX: u8 = 0x00;
/// 0b01000000
pub const _OP_DIFF: u8 = 0x40;
/// 0b10000000
pub const _OP_LUMA: u8 = 0x80;
/// 0b11000000
pub const OP_RUN: u8 = 0xc0;
/// 0b11111110
pub const _OP_RGB: u8 = 0xfe;
/// 0b11111111
pub const _OP_RGBA: u8 = 0xff;
/// 0b11000000
pub const _MASK_2: u8 = 0xc0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
  }
}
