mod algorithm;

use std::error::Error;
use std::io;
use image::{imageops::FilterType,GenericImageView, DynamicImage};

//To do: Hungarian algorithm

fn main() {
    downscale_crop();
}

fn downscale_crop() -> DynamicImage{
    //crop and downscale image
    let file_in = get_args();
    let img = image::open(file_in).unwrap();
    let (width, height) = img.dimensions();
    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;

    let cropped = img.crop_imm(x, y, size, size);
    cropped.resize(256, 256, FilterType::Lanczos3)
}
fn get_args() -> String {
    let mut infile = String::new();
    io::stdin()
        .read_line(&mut infile)
        .expect("Failed to read line");
    infile
}