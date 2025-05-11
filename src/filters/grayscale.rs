//
// Created by J. Blackburn - May 11 2025
//

use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct Grayscale;

pub fn construct(_key_string: &str) -> Box<dyn Filter> {
    Box::new(Grayscale)
}

impl Filter for Grayscale {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();

        for y in 0..height {
        for x in 0..width  {

            let pixel = image_buffer.get_pixel(x, y);
            let new_pixel = grayscale_pixel(pixel);

            image_buffer.put_pixel(x, y, new_pixel);
        }
        } // end loops
    }
}

fn grayscale_pixel(pixel: &Rgb<u8>) -> Rgb<u8> {

    let avg = ( ( pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 ) / 3 ) as u8; 
    Rgb([avg, avg, avg])
}

