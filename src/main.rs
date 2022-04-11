use std::fs;
// mod qoi;

#[derive(Debug, Clone, Copy)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

impl Color {
  fn default() -> Color {
    Color {
      r: 0,
      g: 0,
      b: 0,
      a: 0,
    }
  }

  fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
  }
}

fn main() -> Result<(), std::io::Error> {
  const WIDTH: u32 = 735;
  const HEIGHT: u32 = 588;
  const CHANNELS: u8 = 4;
  // const COLORSPACE: u8 = 0;
  const HEADER_SIZE: u8 = 14;

  let buffer: Vec<u8> = fs::read("assets/monument.bin")?;

  let image_size = buffer.len();
  let last_pixel = image_size - CHANNELS as usize;

  let previous_color = Color::new(0, 0, 0, 255);
  let seen_pixels = [Color::default(); 64];

  const MAX_SIZE: u32 = WIDTH * HEIGHT * (CHANNELS as u32 + 1) + HEADER_SIZE as u32;

  println!("Size: {}", &image_size);
  // println!("{:?}", buffer);

  let mut offset = 0;
  while offset < last_pixel {
    offset += CHANNELS as usize;

    // print!("{offset}, ");
  }

  Ok(())
}
