use image::Rgba;

// The Arecibo message was 23 columns by 73 rows. Or more accurately, it was really a sequence of 1679 bits,
// which can be interpreted as a 23x73 image.
pub const WIDTH: usize = 23;
pub const HEIGHT: usize = 73;

// An array the size of the original image that stores color information.
// A 23x73 pixel image is way too small for web browsers. Once we fill this out, we scale it up
// before generating the actual output iamge.
pub type AbstractImage = [[Rgba<u8>; WIDTH]; HEIGHT];

// Earth's population as depicted in the original message
pub const POPULATION_1974: u128 = 4_292_853_750;

// A population larger than this will run into the planet image
pub const PLANET_MAX_POPULATION: u128 = 281_474_976_710_655;

// A population larger than this will run into the telescope image
pub const TELESCOPE_MAX_POPULATION: u128 = 302_231_454_903_657_293_676_543;

// The estimated size of the human genome in 1974, as depicted in the original message
pub const GENOME_SIZE_1974: u64 = 4_294_441_822;

// Genome sizes larger than this will collide with the least significant bit marker
pub const MAX_GENOME_SIZE: u64 = 4_294_967_295;
