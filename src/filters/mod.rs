//
// Created By J. Blackburn - Jan 20 2025
//

pub mod swap;
pub mod invert;

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub trait Filter {

    fn apply(&self, image: RgbImageBuffer);

}
