use std::fs;
mod qoi;

fn main() -> Result<(), std::io::Error> {
  // Temporary header configuration
  const WIDTH: u32 = 735; // 0x000002df
  const HEIGHT: u32 = 588; // 0x0000024c
  const CHANNELS: u8 = 4; // 0x04
  const COLORSPACE: u8 = 1; // 0x01

  let input: Vec<u8> = fs::read("./assets/monument.bin")?;

  let image_size = input.len();
  let max_size =
    WIDTH as usize * HEIGHT as usize * (CHANNELS as usize + 1) + qoi::HEADER_SIZE + qoi::PADDING;
  let last_pixel = &image_size - CHANNELS as usize;

  let print_width = 15;
  println!("File Size:\t{:>print_width$} bytes", &image_size);
  println!("Max Size:\t{:>print_width$} bytes", &max_size);

  let mut output = Vec::with_capacity(max_size);

  // Write the header
  output.extend_from_slice(&qoi::MAGIC);
  output.extend_from_slice(&WIDTH.to_be_bytes());
  output.extend_from_slice(&HEIGHT.to_be_bytes());
  output.push(CHANNELS);
  output.push(COLORSPACE);

  let mut seen_pixels: [qoi::Color; 64] = [qoi::Color::default(); 64];
  let mut run: u8 = 0;

  let mut previous_color: qoi::Color = qoi::Color::new(0, 0, 0, 255);
  let mut current_color: qoi::Color;

  let mut offset: usize = 0;
  while offset < last_pixel {
    // Read color for the current pixel from bytes
    current_color = qoi::Color {
      r: input[offset + 0],
      g: input[offset + 1],
      b: input[offset + 2],
      a: if CHANNELS < 4 { input[offset + 3] } else { 255 },
    };

    if current_color == previous_color {
      run += 1;
      if run == 62 || offset == last_pixel {
        output.push(qoi::OP_RUN | (run - 1));
        run = 0;
      }
    } else {
      seen_pixels[{
        (current_color.r as usize * 3
          + current_color.g as usize * 5
          + current_color.b as usize * 7
          + current_color.a as usize * 11)
          % 64
      }] = current_color;

      output.push(current_color.r);
      output.push(current_color.g);
      output.push(current_color.b);
      output.push(current_color.a);
    }

    previous_color = current_color;
    offset += CHANNELS as usize;
  }

  // for color in seen_pixels {
  //   println!("{:?}", color);
  // }

  fs::write("./result.qoi", output)?;

  println!(
    "Compressed Size:{:>print_width$} bytes",
    fs::metadata("./result.qoi")
      .expect("Could not read file metadata")
      .len()
  );

  Ok(())
}
