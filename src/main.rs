// Each dot on braille can be considered as one
// of the bits inside of a byte:
// • (1) • (4)
// • (2) • (5)
// • (3) • (6)
// • (7) • (8)
//
// See: https://www.unicode.org/charts/PDF/U2800.pdf

use std::{env, process, fs, slice};
use stb_image_rust;

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

	let mut image_bytes = fs::read(image_name.unwrap()).unwrap();
	let mut width = 0i32;
	let mut height = 0i32;
	let mut channels = 0i32;

	let image: &mut [u8];
	unsafe {
		let image_ptr = stb_image_rust::stbi_load_from_memory(
			image_bytes.as_mut_ptr(),
			image_bytes.len() as i32,
			&mut width, &mut height,
			&mut channels,
			stb_image_rust::STBI_grey
		);

		image = slice::from_raw_parts_mut(image_ptr, (width*height) as usize);
	}

	println!("{}", image[0]);

	unsafe {
		stb_image_rust::stbi_image_free(image.as_mut_ptr());	// Oh boy, good ol' C
	}

	//let character = char::from_u32(BRAILLE + 0b10010011).unwrap();
	//println!("{:?}", character);
}
