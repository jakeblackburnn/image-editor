// 
// Created by J. Blackburn - Jan 19 2025
//

use crate::filters::swap;

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



const OUTPUT_PATH: &str = "./outputs/out-brg.png"; 

pub fn start(image_buffer: RgbImageBuffer) {

        // clone image buffer contents
    let mut image_clone = image_buffer.clone();
    println!("Continuing in add mode"); 

        // add changes to mutable clone and save to output path
    swap::apply("brg", &mut image_clone);

    let _ = image_clone.save(OUTPUT_PATH);

}   
