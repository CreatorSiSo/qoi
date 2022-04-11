#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub magic: [char; 4], // magic bytes "qoif"
  pub width: u32,       // image width in pixels (BE)
  pub height: u32,      // image height in pixels (BE)
  pub channels: u8,     // 3 = RGB, 4 = RGBA
  pub colorspace: u8,   // 0 = sRGB with linear alpha; 1 = all channels linear
}

impl Header {
  // pub fn default() -> Header {
  //   Header {
  //     magic: ['q', 'o', 'i', 'f'],
  //     width: 0,
  //     height: 0,
  //     channels: 3,
  //     colorspace: 0,
  //   }
  // }

  pub fn new(width: u32, height: u32, channels: u8, colorspace: u8) -> Header {
    Header {
      magic: ['q', 'o', 'i', 'f'],
      width,
      height,
      channels,
      colorspace,
    }
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::qoi;

//   #[test]
//   fn default_header() {
//     todo!()
//   }

//   #[test]
//   fn encode() {}

//   #[test]
//   fn decode() {}
// }
