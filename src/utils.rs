use crate::colors::*;
use crate::constants::{AbstractImage, HEIGHT, WIDTH};
use image::{ImageBuffer, Rgba};

// Makes colored pixels gray. Used just to make some images where I highlight two elements
pub fn gray_out_nonblack_pixels(img: &mut AbstractImage) {
    for row in img.iter_mut() {
        for pixel in row.iter_mut() {
            if *pixel != BLACK {
                *pixel = GRAY;
            }
        }
    }
}

// Sets colors of pixels in the abstract image
pub fn write_chunk(
    bits: &[&[u8]],
    img: &mut AbstractImage,
    x_offset: usize,
    y_offset: usize,
    color: Rgba<u8>,
) {
    for (y, row) in bits.iter().enumerate() {
        for (x, bit) in row.iter().enumerate() {
            if *bit == 1 {
                img[y + y_offset][x + x_offset] = color;
            }
        }
    }
}

// Writes the population as a binary string spread across two columns. Starts with the most significant digit on the
// top right going down, then starting back on the top left going down for the second set of 16
// bits.
pub fn write_population_size(
    img: &mut AbstractImage,
    size: Vec<u8>,
    x_offset: usize,
    y_offset: usize,
    color: Rgba<u8>,
) {
    for (n, bit) in size.iter().rev().enumerate() {
        let x = n % 6;
        let y = n / 6;
        if *bit == 1 {
            img[y_offset + y][x_offset + x] = color;
        };
    }
}

// Writes the genome size in a 6xN block, starting with the least significant bit at the top left.
// Reads left to right, then down.
pub fn write_genome_size(
    img: &mut AbstractImage,
    size: Vec<u8>,
    x_offset: usize,
    y_offset: usize,
    color: Rgba<u8>,
) {
    for (n, bit) in size.iter().skip(16).enumerate() {
        if *bit == 1 {
            img[y_offset + n][x_offset] = color;
        }
    }

    for (n, bit) in size.iter().take(16).enumerate() {
        if *bit == 1 {
            img[y_offset + n][x_offset + 1] = color;
        }
    }
}

pub fn create_real_image(img: AbstractImage, scale: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Instantiate the actual output image's buffer, with each pixel initially being black
    let mut out_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(WIDTH as u32 * scale, HEIGHT as u32 * scale, BLACK);

    // Make large colored blocks for each pixel in the abstract image. The blocks are (scale - 2)
    // pixels on each side, with a one-pixel gap between adjacent blocks.
    for (y, row) in img.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            for sub_x in 1u32..(scale - 1) {
                for sub_y in 1u32..(scale - 1) {
                    let actual_x = x as u32 * scale + sub_x;
                    let actual_y = y as u32 * scale + sub_y;
                    let pixel = out_image.get_pixel_mut(actual_x, actual_y);
                    *pixel = *color;
                }
            }
        }
    }
    out_image
}
