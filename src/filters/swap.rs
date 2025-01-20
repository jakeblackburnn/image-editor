// 
// Created By J. Blackburn - Jan 20 2025
//

use image::{ImageBuffer, Rgb};
use regex::Regex;
use std::collections::HashMap;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn apply(key_string: &str, image_buffer: &mut RgbImageBuffer) {

    println!("applying swap: {}", key_string);

        // capture swap pattern from string with regex
    let re = Regex::new(r"(?P<r>[rgb])(?P<g>[rgb])(?P<b>[rgb])")
                    .unwrap();

        // if keys exist apply swap for each pixel
    if let Some(keys) = re.captures(key_string) {
        let rkey = keys.name("r").unwrap().as_str();
        let gkey = keys.name("g").unwrap().as_str();
        let bkey = keys.name("b").unwrap().as_str();

    let (width, height) = image_buffer.dimensions();

    for y in 0..height {
    for x in 0..width  {

        let pixel     = image_buffer.get_pixel(x, y);
        let new_pixel = swap_pixel(pixel, rkey, gkey, bkey);

        image_buffer.put_pixel(x, y, new_pixel);

    }
    } // end loops

    } else {
        panic!("failed to parse swap key");
    }

}


fn swap_pixel(pixel: &Rgb<u8>, rkey: &str, gkey: &str, bkey: &str) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let key_map = HashMap::from([
        ("r" , r),
        ("g" , g),
        ("b" , b),
    ]);

    Rgb([ key_map[rkey], key_map[gkey], key_map[bkey] ])
}
