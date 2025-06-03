//
// Created by J. Blackburn - Apr 25 2025
//

use super::{Filter, FilterError};

use image::{ImageBuffer, Rgb};

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type FilterResult  = Result< Box<dyn Filter>, FilterError >;



pub struct BlackWhite {
    split: u8,
}

impl Filter for BlackWhite {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {
        let dark_pixel  = Rgb([0,0,0]);
        let light_pixel = Rgb([255,255,255]);

        let (width, height) = image_buffer.dimensions();
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

    fn construct(key: &str) -> FilterResult {
        let split: u8 = match key.parse() {
            Ok(x) => x,
            Err(_) => return Err(FilterError::FilterConstructionFailed),
        };
        Ok( Box::new( BlackWhite { split } ) )
    }
}



fn is_dark_pixel(pixel: &Rgb<u8>, split: u8) -> bool {
    let avg = ( ( pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 ) / 3 ) as u8; 
    avg < split
}
