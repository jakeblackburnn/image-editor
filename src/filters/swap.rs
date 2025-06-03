// 
// Created By J. Blackburn - Jan 20 2025
//

use super::{Filter, FilterError}; 

use image::{ImageBuffer, Rgb};
use regex::Regex;
use std::collections::HashMap;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type FilterResult = Result< Box<dyn Filter>, FilterError>;



pub struct Swap {
    rkey: char,
    gkey: char,
    bkey: char,
}

impl Filter for Swap {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();
        for y in 0..height {
        for x in 0..width  {
            let pixel     = image_buffer.get_pixel(x, y);
            let new_pixel = swap_pixel(pixel, self.rkey, self.gkey, self.bkey);

            image_buffer.put_pixel(x, y, new_pixel);
        }
        } // end loops
    }

    fn construct(key: &str) -> FilterResult {
        let re = Regex::new(r"(?P<r>[rgb])(?P<g>[rgb])(?P<b>[rgb])")
                        .unwrap();

        if let Some(c) = re.captures(key) {
            let rkey: char = c.name("r").unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let gkey: char = c.name("g").unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let bkey: char = c.name("b").unwrap().as_str().chars().collect::<Vec<char>>()[0];

            Ok( Box::new(Swap {
                rkey,
                gkey,
                bkey,
            }))
        } else {
            Err(FilterError::InvalidKey)
        }
    }
}



fn swap_pixel(pixel: &Rgb<u8>, rkey: char, gkey: char, bkey: char) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let key_map = HashMap::from([
        ('r' , r),
        ('g' , g),
        ('b' , b),
    ]);
    Rgb([ key_map[&rkey], key_map[&gkey], key_map[&bkey] ])
}
