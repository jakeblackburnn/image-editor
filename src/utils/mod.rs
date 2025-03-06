// 
// Created by J. Blackburn - Mar 5 2025
//


use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn load_rgb_image_buffer(input_image: String) -> RgbImageBuffer {

    image::open(input_image.as_str())
            .expect("failed to open image buffer")
            .to_rgb8()

}
