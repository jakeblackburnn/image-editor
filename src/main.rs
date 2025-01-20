//
// Created by J. Blackburn - Jan 11 2025
//
 
mod add;
mod view;

use std::env;
use image;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mode       = args.get(1).map(String::as_str).expect("no mode specified");
    let image_path = args.get(2).map(String::as_str).expect("no image path specified");



    println!("Attempting to load image buffer: {}", image_path);
    let image_buffer = image::open(image_path)
                            .expect("failed to open image buffer")
                            .to_rgb8();

    match mode {

        "add" => {
            add::start();
        }

        "view" => {
            view::start();
        }

        _ => {
            println!("mode argument invalid");
        }
    }
}
