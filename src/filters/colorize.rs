//
// Created by J. Blackburn - May 12 2025
//

use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};
use regex::Regex;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct Colorize {
    cur: Rgb<u8>,
    sub:   Rgb<u8>,
}

pub fn construct(key_string: &str) -> Box<dyn Filter> {

    let re = Regex::new(
    r"(?<cr>\d{1,3})\.(?<cg>\d{1,3})\.(?<cb>\d{1,3}),(?<sr>\d{1,3})\.(?<sg>\d{1,3})\.(?<sb>\d{1,3})" 
    )
                    .unwrap();

    let captures = re.captures(key_string).expect("Failed to parse colorize key string");

    let cur_r = captures.name("cr").unwrap().as_str();
    let cur_g = captures.name("cg").unwrap().as_str();
    let cur_b = captures.name("cb").unwrap().as_str();
    let sub_r = captures.name("sr").unwrap().as_str();
    let sub_g = captures.name("sg").unwrap().as_str();
    let sub_b = captures.name("sb").unwrap().as_str();

    let cur_r: u8 = cur_r.parse().expect("Failed to parse key string to u8");
    let cur_g: u8 = cur_g.parse().expect("Failed to parse key string to u8");
    let cur_b: u8 = cur_b.parse().expect("Failed to parse key string to u8");
    let sub_r: u8 = sub_r.parse().expect("Failed to parse key string to u8");
    let sub_g: u8 = sub_g.parse().expect("Failed to parse key string to u8");
    let sub_b: u8 = sub_b.parse().expect("Failed to parse key string to u8");

    Box::new( Colorize { cur: Rgb([cur_r, cur_g, cur_b]), sub: Rgb([sub_r, sub_g, sub_b]) } )
}

impl Filter for Colorize {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();

        for y in 0..height {
        for x in 0..width  {

            let pixel = image_buffer.get_pixel(x, y);

            let cur = self.cur;
            let sub = self.sub;

            if *pixel == cur {
                image_buffer.put_pixel(x, y, sub);
            }
        }
        } // end loops
    }
}
