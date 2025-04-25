//
// Created by J. Blackburn - Apr 25 2025
//


use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct BlackWhite {
    split: u8,
}

pub fn construct(key_string: &str) -> Box<dyn Filter> {

    let split: u8 = key_string.parse().expect("Failed to parse key string to u8");

    Box::new( BlackWhite { split } )
}

impl Filter for BlackWhite {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();

        let dark_pixel  = Rgb([0,0,0]);
        let light_pixel = Rgb([255,255,255]);

        for y in 0..height {
        for x in 0..width  {

            let pixel     = image_buffer.get_pixel(x, y);


            if ( is_dark_pixel(pixel, self.split) ) {
                image_buffer.put_pixel(x, y, dark_pixel);
            } else {
                image_buffer.put_pixel(x, y, light_pixel);
            }


        }
        } // end loops
    }
}

fn is_dark_pixel(pixel: &Rgb<u8>, split: u8) -> bool {
    let avg = ( ( pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 ) / 3 ) as u8; 
    avg < split
}

