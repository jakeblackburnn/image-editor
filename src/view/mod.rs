// 
// Created By J. Blackburn - Jan 20 2025
//

use crate::utils::load_rgb_image_buffer;

use eframe::egui;
use image::{ImageBuffer, Rgb};

use std::sync::{Arc, Mutex};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type SharedImageBuffer = Arc<Mutex<RgbImageBuffer>>;



pub fn start(mut args: std::env::Args) {

        // load base image buffer
    let input_image = args.next().expect("Failed to parse image path");
    let image_buffer = load_rgb_image_buffer(input_image);

        // create shareable clone of image
    let mut sharedImageBuffer = image_buffer.clone();

    println!("continuing in view mode");

}
