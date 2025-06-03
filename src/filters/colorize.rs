//
// Created by J. Blackburn - May 12 2025
//

use super::{Filter, FilterError}; 

use image::{ImageBuffer, Rgb};
use regex::Regex;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type FilterResult  = Result< Box<dyn Filter>, FilterError >;



pub struct Colorize {
    cur: Rgb<u8>,
    sub: Rgb<u8>,
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

    fn construct(key: &str) -> FilterResult {
        let re = Regex::new(
    r"(?<cr>\d{1,3})\.(?<cg>\d{1,3})\.(?<cb>\d{1,3}),(?<sr>\d{1,3})\.(?<sg>\d{1,3})\.(?<sb>\d{1,3})" 
        ).unwrap();

        if let Some(c) = re.captures(key) {
            let cur_r = c.name("cr").unwrap().as_str();
            let cur_g = c.name("cg").unwrap().as_str();
            let cur_b = c.name("cb").unwrap().as_str();
            let sub_r = c.name("sr").unwrap().as_str();
            let sub_g = c.name("sg").unwrap().as_str();
            let sub_b = c.name("sb").unwrap().as_str();

            let cur_r: u8 = match cur_r.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };
            let cur_g: u8 = match cur_g.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };
            let cur_b: u8 = match cur_b.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };
            let sub_r: u8 = match sub_r.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };
            let sub_g: u8 = match sub_g.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };
            let sub_b: u8 = match sub_b.parse() {
                Ok(x) => x,
                Err(_) => return Err(FilterError::FilterConstructionFailed),
            };

            Ok(
        Box::new( Colorize { cur: Rgb([cur_r, cur_g, cur_b]), sub: Rgb([sub_r, sub_g, sub_b]) } )
            )
        } else {
            Err(FilterError::InvalidKey)
        }
    }
}
