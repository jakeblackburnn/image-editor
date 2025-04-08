//
// Created by J. Blackburn - Apr 7 2025
//


use super::Filter; // pull down filter trait from mod.rs

use image::{ImageBuffer, Rgb};
use regex::Regex;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;


pub struct Plus {
    n: u8,
    color: Option<char>,
}

pub fn construct(key_string: &str) -> Box<dyn Filter> {

    let re = Regex::new(r"(?P<color>[rgb])?(?P<n>[0-9]+)")
                    .unwrap();

    let captures = re.captures(key_string).expect("Failed to parse plus key string");

    let n_str = captures.name("n").unwrap().as_str();

    let color = captures.name("color").map(|c| c.as_str()
                                                .chars()
                                                .next()
                                                .unwrap() );



    let n: u8 = n_str.parse().expect("Failed to parse key string to u8");
    Box::new( Plus { n, color } )
}

impl Filter for Plus {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();

        for y in 0..height {
        for x in 0..width  {

            let pixel     = image_buffer.get_pixel(x, y);

            let mut new_pixel = Rgb([0,0,0]);

            match self.color { 
                None      => { new_pixel = add_to_pixel(pixel, self.n); },
                Some('r') => { new_pixel = add_to_r(pixel, self.n); },
                Some('g') => { new_pixel = add_to_g(pixel, self.n); },
                Some('b') => { new_pixel = add_to_b(pixel, self.n); },
                _ => { panic!("plus failed due to unexpected color value!?!?"); },
            }

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

fn add_to_r(pixel: &Rgb<u8>, n: u8) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let newr = r.saturating_add(n);

    Rgb([ newr, g, b ])
}

fn add_to_g(pixel: &Rgb<u8>, n: u8) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let newg = g.saturating_add(n);

    Rgb([ r, newg, b ])
}

fn add_to_b(pixel: &Rgb<u8>, n: u8) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let newb = b.saturating_add(n);

    Rgb([ r, g, newb ])
}

