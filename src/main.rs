// Each dot on braille can be considered as one
// of the bits inside of a byte:
// • (1) • (4)
// • (2) • (5)
// • (3) • (6)
// • (7) • (8)
//
// See: https://www.unicode.org/charts/PDF/U2800.pdf

use std::{env, process, fs};

const BRAILLE: u32 = 0x2800;

fn usage(executable: &str) {
	print!(
		concat!(
			"Usage: {} [OPTION]... FILE\n",
			"Convert an image to a braille ascii art.\n\n",
			"OPTIONS:\n",
			" -h, --help         show this help message\n",
			" -i                 invert the colors\n",
			" -w, --width=WIDTH  change the size of the image\n"
		),
		executable
	);
}

fn main() {
	let mut image_name = None;
	let mut inverted = false;
	let mut width = None;

	let mut args = env::args();
	let executable = args.next().unwrap();
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--help" | "-h" => {
				usage(&executable);
				process::exit(0);
			},
			"-i" => {
				inverted = true;
			},
			"-w" | "--width" => {
				if let Some(param) = args.next() {
					width = Some(param.parse::<u32>().unwrap());
				}
			},
			_ => {
				image_name = Some(arg);
			}
		}
	}

	if image_name.is_none() {
		usage(&executable);
		process::exit(1);
	}

	let mut image = image::load_from_memory(
		&fs::read(image_name.unwrap()).unwrap()
	).unwrap().into_luma8();

	if let Some(width) = width {
		image = image::imageops::resize(
			&image,
			width, width * image.height() / image.width(),
			image::imageops::FilterType::Triangle
		);
	}

	let image_pixels = image.as_raw();
	let image_width = image.width() as usize;
	let image_height = image.height() as usize;
	for y in (0..image_height).step_by(4) {
		for x in (0..image_width).step_by(2) {
			let mut top_pixels = Vec::new();
			let mut bottom_pixels = Vec::new();

			for x_offset in 0..=1 {
				for y_offset in 0..=3 {
					let mut pixel = None;
					if x + x_offset < image_width && y + y_offset < image_height {
						pixel = Some(
							image_pixels[image_width * (y + y_offset) + (x + x_offset)]
						);
					}

					if y_offset < 3 {
						top_pixels.push(pixel);
					} else {
						bottom_pixels.push(pixel);
					}
				}
			}

			top_pixels.append(&mut bottom_pixels);

			let mut byte = 0u8;
			for i in 0..top_pixels.len() {
				if let Some(pixel) = top_pixels[i] {
					let bit = ((!inverted && pixel < 128) || (inverted && pixel >= 128)) as u8;
					byte |= bit << i;
				}
			}

			print!("{}", char::from_u32(BRAILLE + byte as u32).unwrap());
		}
		print!("\n");
	}
}
