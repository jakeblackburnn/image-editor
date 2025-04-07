//
// Created by J. Blackburn - Apr 7 2025
//


use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct Plus {
    n: u8,
}

pub fn construct(key_string: &str) -> Box<dyn Filter> {
    let n: u8 = key_string.parse().expect("Failed to parse key string to u32");

    Box::new( Plus { n } )
}

impl Filter for Plus {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();

        for y in 0..height {
        for x in 0..width  {

            let pixel     = image_buffer.get_pixel(x, y);
            let new_pixel = add_to_pixel(pixel, self.n);

            image_buffer.put_pixel(x, y, new_pixel);

        }
        } // end loops
    }
}


fn add_to_pixel(pixel: &Rgb<u8>, n: u8) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let newr = r.saturating_add(n);
    let newg = g.saturating_add(n);
    let newb = b.saturating_add(n);

    Rgb([ newr, newg, newb ])
}
