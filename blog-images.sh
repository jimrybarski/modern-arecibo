#!/usr/bin/env bash

# Generates images for a blog post about this tool.

set -euo pipefail

cargo build --release

t2t="3117275501" # https://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/009/914/755/GCF_009914755.1_T2T-CHM13v2.0/GCF_009914755.1_T2T-CHM13v2.0_genomic.fna.gz
population_2025="8098171861" 
max_pop_six_rows="68719476735"
max_pop="281474976710655"
super_max_pop="302231454903657293676543"

# remake the original image
target/release/modern-arecibo --output "images/original.png"

# original image with the genome and population highlighted
target/release/modern-arecibo --output "images/highlight.png" --highlight-population --highlight-genome

# modern image
target/release/modern-arecibo --output "images/modern.png" --population $population_2025 --genome $t2t

# modern image without Pluto
target/release/modern-arecibo --output "images/modern-no-pluto.png" --population $population_2025 --genome $t2t --pluto-is-not-a-planet

# image with the maximum possible population (assuming six rows maximum) highlighted
target/release/modern-arecibo --output "images/maxpop-six-rows.png" --highlight-population --population $max_pop_six_rows --genome $t2t

# image with the maximum possible population highlighted. Any larger than this and the number would bleed into the planet depiction just below it
target/release/modern-arecibo --output "images/maxpop.png" --highlight-population --population $max_pop --genome $t2t

# image with the maximum possible population highlighted, if we eliminate the planets past Jupiter.
target/release/modern-arecibo --output "images/supermaxpop.png" --highlight-population --population $super_max_pop --genome $t2t
