use image::{ImageBuffer, Rgba};

const ZERO_ASCII: u8 = 48;
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([213, 94, 0, 255]);
const BLUE: Rgba<u8> = Rgba([0, 114, 178, 255]);
const YELLOW: Rgba<u8> = Rgba([240, 228, 66, 255]);
const ORANGE: Rgba<u8> = Rgba([230, 159, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 158, 115, 255]);
const PURPLE: Rgba<u8> = Rgba([204, 121, 167, 255]);

fn write_rows(
    rows: &[Vec<u8>],
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
    x_offset: usize,
    y_offset: usize,
) {
    for (y, row) in rows.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let color = if *value == 1 { color } else { BLACK };
            let pixel = img.get_pixel_mut(
                (x + x_offset).try_into().unwrap(),
                (y + y_offset).try_into().unwrap(),
            );
            *pixel = color;
        }
    }
}

fn write_chunk(
    bits: &[Vec<u8>],
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x_offset: usize,
    y_offset: usize,
    color: Rgba<u8>,
) {
    for (y, row) in bits.iter().enumerate() {
        for (x, bit) in row.iter().enumerate() {
            let pixel = img.get_pixel_mut(
                (x + x_offset).try_into().unwrap(),
                (y + y_offset).try_into().unwrap(),
            );
            if *bit == 1 {
                *pixel = color;
            }
        }
    }
}

fn write_population_size(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    size: Vec<u8>,
    x_offset: usize,
    y_offset: usize,
) {
    for (n, bit) in size.iter().rev().enumerate() {

        let x = n % 6;
        let y = n / 6;
        let pixel = img.get_pixel_mut(
            (x_offset + x).try_into().unwrap(),
            (y_offset + y).try_into().unwrap(),
        );
        if *bit == 1 {
            *pixel = WHITE;
        };
    }
}

fn write_genome_size(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    size: Vec<u8>,
    x_offset: usize,
    y_offset: usize,
) {
    for (n, bit) in size.iter().skip(16).enumerate() {
        let pixel = img.get_pixel_mut(
            (x_offset).try_into().unwrap(),
            (y_offset + n).try_into().unwrap(),
        );
        if *bit == 1 {
            *pixel = WHITE;
        }
    }

    for (n, bit) in size.iter().take(16).enumerate() {
        let pixel = img.get_pixel_mut(
            (x_offset + 1).try_into().unwrap(),
            (y_offset + n).try_into().unwrap(),
        );
        if *bit == 1 {
            *pixel = WHITE;
        }
    }
}

fn main() {
    let width = 23u8;
    let height = 73u8;

    let one_to_ten = [
        vec![
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
        vec![
            1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0,
        ],
        vec![
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0,
        ],
    ];

    let dna_elements = [
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ];

    let deoxyribose: Vec<Vec<u8>> = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let adenine: Vec<Vec<u8>> = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let thymine: Vec<Vec<u8>> = vec![
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let cytosine: Vec<Vec<u8>> = vec![
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let guanine: Vec<Vec<u8>> = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1],
    ];

    let phosphate: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];

    let dna_helix = [
        vec![
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
        vec![
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ],
        vec![
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
        vec![
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
        vec![
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ],
        vec![
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ];

    let little_guy: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 1, 1, 0, 1, 0],
        vec![1, 0, 0, 1, 1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 1, 1, 0, 0],
    ];

    let ruler: Vec<Vec<u8>> = vec![
        vec![1],
        vec![1],
        vec![1],
        vec![1],
        vec![0],
        vec![0],
        vec![0],
        vec![1],
        vec![1],
        vec![1],
    ];

    let fourteen: Vec<Vec<u8>> = vec![vec![1, 0, 1, 1, 1]];

    let planets: Vec<Vec<u8>> = vec![
        vec![
            1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![
            1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
        ],
        vec![
            1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
        ],
        vec![
            1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
        ],
    ];

    let telescope: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let blue_ground = [vec![1, 1, 1, 1]];

    let telescope_size = vec![vec![1, 0, 0, 1, 0, 1, 0], vec![1, 1, 1, 1, 1, 0, 1]];

    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::new(width.into(), height.into());

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel_mut(x.into(), y.into());
            *pixel = BLACK;
        }
    }

    let genome_size: u32 = 3_100_000_000;
    let genome_binary = format!("{genome_size:b}")
        .chars()
        .map(|bs| bs as u8 - ZERO_ASCII)
        .collect::<Vec<u8>>();

    // let population: u64 = 68719476735; // Maximum possible with six rows
    // let population: u64 = 281474976710655; // Maximum possible
    let population: u64 = 8_203_268_787; // Actual
    let population_binary = format!("{population:064b}")
        .chars()
        .map(|bs| bs as u8 - ZERO_ASCII)
        .collect::<Vec<u8>>();

    let pixel = img.get_pixel_mut(16, 48);
    *pixel = WHITE;

    write_population_size(&mut img, population_binary, 17, 48);

    write_rows(&one_to_ten, &mut img, WHITE, 0, 0);
    write_rows(&dna_elements, &mut img, PURPLE, 0, one_to_ten.len());

    write_chunk(&deoxyribose, &mut img, 0, 11, GREEN);
    write_chunk(&adenine, &mut img, 6, 11, GREEN);
    write_chunk(&thymine, &mut img, 12, 11, GREEN);
    write_chunk(&deoxyribose, &mut img, 18, 11, GREEN);

    write_chunk(&phosphate, &mut img, 0, 16, GREEN);
    write_chunk(&phosphate, &mut img, 18, 16, GREEN);

    write_chunk(&deoxyribose, &mut img, 0, 21, GREEN);
    write_chunk(&cytosine, &mut img, 6, 21, GREEN);
    write_chunk(&guanine, &mut img, 12, 21, GREEN);
    write_chunk(&deoxyribose, &mut img, 18, 21, GREEN);

    write_chunk(&phosphate, &mut img, 0, 26, GREEN);
    write_chunk(&phosphate, &mut img, 18, 26, GREEN);

    write_genome_size(&mut img, genome_binary, 10, 26);
    let pixel = img.get_pixel_mut(10, 42);
    *pixel = WHITE;

    write_chunk(&dna_helix, &mut img, 0, 31, BLUE);

    write_chunk(&little_guy, &mut img, 6, 45, RED);
    write_chunk(&ruler, &mut img, 2, 45, BLUE);
    write_chunk(&fourteen, &mut img, 0, 50, WHITE);
    write_chunk(&planets, &mut img, 2, 56, YELLOW);
    write_chunk(&telescope, &mut img, 1, 60, PURPLE);
    write_chunk(&blue_ground, &mut img, 1, 72, BLUE);
    write_chunk(&blue_ground, &mut img, 16, 72, BLUE);
    write_chunk(&telescope_size, &mut img, 7, 71, WHITE);

    img.save("modern-arecibo.png")
        .expect("Failed to save image");
}
