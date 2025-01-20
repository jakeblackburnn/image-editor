// 
// Created By J. Blackburn - Jan 20 2025
//

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn apply(swap_key: &str, image_buffer: &mut RgbImageBuffer) {

    println!("applying swap: {}", swap_key);

}
