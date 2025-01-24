//
// Created by J. Blackburn - Jan 11 2025
//
 
mod add;
mod view;
mod filters;

use std::env;
use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


fn main() {

    let args: Vec<String> = env::args().collect();

        // Required Arguments
        // mode and image path are always 1st and 2nd arguments
    let mode       = args.get(1).map(String::as_str).expect("no mode specified");
    let image_path = args.get(2).map(String::as_str).expect("no image path specified");

        // Optional Arguments
        // nothing here yet...

    println!("Attempting to load image buffer: {}", image_path);
    let image_buffer: RgbImageBuffer = image::open(image_path)
                                .expect("failed to open image buffer")
                                .to_rgb8();

    match mode {

        "add" => {
            add::start(image_buffer);
        }

        "view" => {
            view::start(image_buffer);
        }

        _ => {
            panic!("mode argument invalid");
        }
    }
}
