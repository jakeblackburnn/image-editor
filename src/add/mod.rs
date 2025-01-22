// 
// Created by J. Blackburn - Jan 19 2025
//

use crate::filters::swap;
use crate::filters::invert;

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


const OUTPUT_PATH: &str = "./outputs/matts-creek-fire-invert-rbb.png"; 

pub fn start(image_buffer: RgbImageBuffer) {

        // clone image buffer contents
    let mut image_clone = image_buffer.clone();
    println!("Continuing in add mode"); 

        // add changes to mutable clone and save to output path
    swap::apply("rbb", &mut image_clone);
    invert::apply(&mut image_clone);


    let _ = image_clone.save(OUTPUT_PATH);

}   
