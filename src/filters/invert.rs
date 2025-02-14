// 
// Created By J. Blackburn - Jan 20 2025
//

use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct Invert;


pub fn construct() -> Box<dyn Filter> {
    Box::new(Invert) 
}

impl Filter for Invert {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {
       
        println!("applying invert");

        let (width, height) = image_buffer.dimensions();

        for y in 0..height {
        for x in 0..width  {

            let pixel     = image_buffer.get_pixel(x, y);
            let new_pixel = invert_pixel(pixel);

            image_buffer.put_pixel(x, y, new_pixel);

        }
        } 

    }

}

fn invert_pixel(pixel: &Rgb<u8>) -> Rgb<u8> {

    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    Rgb([ 255 - r, 255 - g, 255 - b ])

}
