// 
// Created by J. Blackburn - Jan 19 2025
//

use image::{ImageBuffer, Rgb};
use crate::filters;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub fn start(mut image_buffer: RgbImageBuffer, filter: Option<&String>, save_path: Option<&String>) {

    let factory = filters::FilterFactory::new();

    let invert = factory.get("invert");

    invert().apply(&mut image_buffer);

    println!("saving image. . .");
    image_buffer.save("outputs/out.png");
    println!("image saved.");
}   
