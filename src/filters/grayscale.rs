//
// Created by J. Blackburn - May 11 2025
//

use super::{Filter, FilterError}; 

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type FilterResult = Result<Box<dyn Filter>, FilterError>;



pub struct Grayscale;

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

    fn construct(_key: &str) -> FilterResult {
        Ok( Box::new(Grayscale) )
    }
}



fn grayscale_pixel(pixel: &Rgb<u8>) -> Rgb<u8> {
    let avg = ( ( pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 ) / 3 ) as u8; 
    Rgb([avg, avg, avg])
}

