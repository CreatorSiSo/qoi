use std::fs;
mod qoi;

fn main() -> Result<(), std::io::Error> {
  // Temporary header configuration
  const WIDTH: u32 = 735;
  const HEIGHT: u32 = 588;
  const CHANNELS: u8 = 3; // 0b00000011
  const COLORSPACE: u8 = 1; // 0b00000001

  let input: Vec<u8> = fs::read("./assets/monument-rgb.bin")?;

  let image_size = input.len();
  let _max_size =
    WIDTH as usize * HEIGHT as usize * (CHANNELS as usize + 1) + qoi::HEADER_SIZE + qoi::PADDING;
  let last_pixel = &image_size - CHANNELS as usize;

  let print_width = 15;
  println!("File Size:\t{:>print_width$} bytes", &image_size);
  println!("Max Size:\t{:>print_width$} bytes", &image_size);

  // TODO: Use calculated max_size again when header is known to be correct
  let mut output = Vec::with_capacity(image_size);

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
      // Maximum run length reached
      if run == 62 || offset == last_pixel {
        output.push(qoi::OP_RUN | (run - 1));
        run = 0;
      }
    } else {
      // The current_color is not the same as the one used by the run
      if run > 0 {
        output.push(qoi::OP_RUN | (run - 1));
        run = 0;
      }

      let current_color_hash = &current_color.to_hash() % 64;
      if current_color == seen_pixels[current_color_hash] {
        output.push(qoi::OP_INDEX | current_color_hash as u8)
      } else {
        seen_pixels[current_color_hash] = current_color;

        let color_diff = previous_color - current_color;
        if (-2..=1).contains(&color_diff.r)
          && (-2..=1).contains(&color_diff.g)
          && (-2..=1).contains(&color_diff.b)
          && color_diff.a == 0
        {
          let r = ((color_diff.r + 2) as u8) << 4;
          let g = ((color_diff.g + 2) as u8) << 2;
          let b = ((color_diff.b + 2) as u8) << 0;
          output.push(qoi::OP_DIFF | r | g | b);
        } else {
          // Write color value
          output.push(if CHANNELS == 4 {
            qoi::OP_RGBA
          } else {
            qoi::OP_RGB
          });
          output.push(current_color.r);
          output.push(current_color.g);
          output.push(current_color.b);
          if CHANNELS == 4 {
            output.push(current_color.a);
          }
        }
      }
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
