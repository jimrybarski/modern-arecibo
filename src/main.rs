use clap::{arg, command};
use std::process::exit;
mod chunks;
mod colors;
mod constants;
mod utils;
use chunks::*;
use colors::*;
use constants::*;
use utils::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = POPULATION_1974)]
    population: u64,
    #[arg(long, default_value_t = GENOME_SIZE_1974)]
    genome: u64,
    #[arg(long, default_value_t = String::from("arecibo.png"))]
    output: String,
    #[arg(long, default_value_t = false)]
    highlight_genome: bool,
    #[arg(long, default_value_t = false)]
    highlight_population: bool,
    #[arg(long, default_value_t = false)]
    pluto_is_not_a_planet: bool,
    #[arg(long, default_value_t = 10)]
    scale: u32,
}

fn main() {
    let args = Args::parse();

    if args.population > MAX_POPULATION {
        eprintln!("Population cannot exceed 281,474,976,710,655");
        exit(1);
    }

    if args.genome > MAX_GENOME_SIZE {
        eprintln!("Genome size cannot exceed 4,294,967,296");
        exit(1);
    }
    let mut img: AbstractImage = [[BLACK; WIDTH]; HEIGHT];

    let genome_size = args.genome;
    let genome_binary = format!("{genome_size:b}")
        .chars()
        .map(|bs| bs as u8 - ZERO_ASCII)
        .collect::<Vec<u8>>();

    let population = args.population;
    let population_binary = format!("{population:064b}")
        .chars()
        .map(|bs| bs as u8 - ZERO_ASCII)
        .collect::<Vec<u8>>();

    write_chunk(ONE_TO_TEN, &mut img, 0, 0, WHITE);
    write_chunk(DNA_ELEMENTS, &mut img, 9, 5, PURPLE);
    write_chunk(DEOXYRIBOSE, &mut img, 0, 11, GREEN);
    write_chunk(ADENINE, &mut img, 6, 11, GREEN);
    write_chunk(THYMINE, &mut img, 12, 11, GREEN);
    write_chunk(DEOXYRIBOSE, &mut img, 18, 11, GREEN);
    write_chunk(PHOSPHATE, &mut img, 0, 16, GREEN);
    write_chunk(PHOSPHATE, &mut img, 18, 16, GREEN);
    write_chunk(DEOXYRIBOSE, &mut img, 0, 21, GREEN);
    write_chunk(CYTOSINE, &mut img, 6, 21, GREEN);
    write_chunk(GUANINE, &mut img, 12, 21, GREEN);
    write_chunk(DEOXYRIBOSE, &mut img, 18, 21, GREEN);
    write_chunk(PHOSPHATE, &mut img, 0, 26, GREEN);
    write_chunk(PHOSPHATE, &mut img, 18, 26, GREEN);
    write_chunk(DNA_HELIX, &mut img, 0, 31, BLUE);
    write_chunk(RULER, &mut img, 2, 45, BLUE);
    write_chunk(FOURTEEN, &mut img, 0, 50, WHITE);
    write_chunk(PLANETS, &mut img, 2, 56, YELLOW);
    write_chunk(TELESCOPE, &mut img, 1, 60, PURPLE);
    write_chunk(BLUE_GROUND, &mut img, 1, 72, BLUE);
    write_chunk(BLUE_GROUND, &mut img, 16, 72, BLUE);
    write_chunk(TELESCOPE_SIZE, &mut img, 7, 71, WHITE);
    write_chunk(LITTLE_GUY, &mut img, 6, 45, RED);

    // There are two pixels that serve as least-significant bit markers
    img[48][16] = WHITE;
    img[42][10] = WHITE;

    if args.highlight_genome || args.highlight_population {
        gray_out_nonblack_pixels(&mut img);
    }

    let (genome_color, population_color) = match (args.highlight_genome, args.highlight_population)
    {
        (true, true) => (BLUE, RED),
        (true, false) => (BLUE, GRAY),
        (false, true) => (GRAY, RED),
        (false, false) => (WHITE, WHITE),
    };
    write_genome_size(&mut img, genome_binary, 10, 26, genome_color);
    write_population_size(&mut img, population_binary, 17, 48, population_color);

    if args.pluto_is_not_a_planet {
        // blasphemy
        img[57][22] = BLACK;
    }

    let out_image = create_real_image(img, args.scale);
    out_image.save(args.output).expect("Failed to save image");
}
