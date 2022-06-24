// Each dot on braille can be considered as one
// of the bits inside of a byte:
// • (1) • (4)
// • (2) • (5)
// • (3) • (6)
// • (7) • (8)
//
// See: https://www.unicode.org/charts/PDF/U2800.pdf

const BRAILLE: u32 = 0x2800;

fn main() {
	let character = char::from_u32(BRAILLE + 0b10010011).unwrap();
	println!("{:?}", character);
}
