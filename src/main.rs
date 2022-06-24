// Each dot on braille can be considered as one
// of the bits inside of a byte:
// • (1) • (4)
// • (2) • (5)
// • (3) • (6)
// • (7) • (8)
//
// See: https://www.unicode.org/charts/PDF/U2800.pdf

use std::{env, process, fs};
use image;

const BRAILLE: u32 = 0x2800;

fn usage(executable: &str) {
	print!(
		concat!(
			"Usage: {} [OPTION]... FILE\n",
			"Convert an image to a braille ascii art.\n\n",
			"OPTIONS:\n",
			" -h, --help       show this help message\n"
		),
		executable
	);
}

fn main() {
	let mut image_name = None;

	let mut args = env::args();
	let executable = args.next().unwrap();
	for arg in args {
		match arg.as_str() {
			"--help" | "-h" => {
				usage(&executable);
				process::exit(0);
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

	let image = image::load_from_memory(
		&fs::read(image_name.unwrap()).unwrap()
	).unwrap().into_luma8();

	let image_stretched = image::imageops::resize(
		&image,
		image.width(), image.height() / 2,
		image::imageops::FilterType::Triangle
	);

	//let character = char::from_u32(BRAILLE + 0b10010011).unwrap();
	//println!("{:?}", character);
}
