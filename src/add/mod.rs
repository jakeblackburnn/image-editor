// 
// Created by J. Blackburn - Jan 19 2025
//

use crate::filters::swap;

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn start(image_buffer: RgbImageBuffer) {

        // clone image buffer contents
    let mut image_clone = image_buffer.clone();
    println!("Continuing in add mode"); 

        // add changes to mutable clone
    swap::apply("brg", &mut image_clone);
    image_clone.save("./outputs/out.png");

}   
