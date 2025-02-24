// 
// Created By J. Blackburn - Jan 20 2025
//

use image::{ImageBuffer, Rgb};
use regex::Regex;
use std::collections::HashMap;

use super::Filter; // pull down filter trait from mod.rs

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub struct Swap {
    rkey: char,
    gkey: char,
    bkey: char,
}

pub fn construct(key_string: &str) -> Box<dyn Filter> {

        let re = Regex::new(r"(?P<r>[rgb])(?P<g>[rgb])(?P<b>[rgb])")
                        .unwrap();

            // get swap pattern from key string
        if let Some(keys) = re.captures(key_string) {

                // get keys as str from regex, convert to char
            let rkey: char = keys.name("r").unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let gkey: char = keys.name("g").unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let bkey: char = keys.name("b").unwrap().as_str().chars().collect::<Vec<char>>()[0];

                // return box w/ filter locked and loaded
            Box::new(Swap {
                rkey,
                gkey,
                bkey,
            })

        } else {
            panic!("failed to parse swap key");
        }
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
