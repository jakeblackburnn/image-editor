//
// Created by J. Blackburn - Apr 8 2025
//

use super::{Filter, FilterError}; 

use image::{ImageBuffer, Rgb};
use regex::Regex;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

type FilterResult = Result<Box<dyn Filter>, FilterError>;



pub struct Mult {
    n: f32,
    color: Option<char>,
}

impl Filter for Mult {

    fn apply(&self, image_buffer: &mut RgbImageBuffer) {

        let (width, height) = image_buffer.dimensions();
        for y in 0..height {
        for x in 0..width  {

            let pixel = image_buffer.get_pixel(x, y);
            let mut new_pixel = Rgb([0,0,0]);

            match self.color  {
                None      => { new_pixel = mult_to_pixel(pixel, self.n); },
                Some('r') => { new_pixel = mult_to_r(pixel, self.n); },
                Some('g') => { new_pixel = mult_to_g(pixel, self.n); },
                Some('b') => { new_pixel = mult_to_b(pixel, self.n); },
                _ => { panic!("Mult failed due to unexpected color value?!?!"); }
            }
            image_buffer.put_pixel(x, y, new_pixel);
        }
        } // end loops
    }

    fn construct(key: &str) -> FilterResult {
        let re = Regex::new(r"(?P<color>[rgb])?(?P<n>[0-9]+\.[0-9]+)")
                        .unwrap();

        if let Some(c) = re.captures(key) {
            let n_str = c.name("n").unwrap().as_str();
            let color = c.name("color").map(|c| c.as_str()
                                                        .chars()
                                                        .next()
                                                        .unwrap() );
            let n: f32 = match n_str.parse() {
                Ok(x) => x,
                Err(_)  => return Err(FilterError::FilterConstructionFailed),
            };

            Ok( Box::new( Mult { n, color } ) ) 
        } else {
            Err(FilterError::InvalidKey)
        }
    }
}



fn mult_color_channel(x: u8, n: f32) -> u8   { (((x as f32) * n ).min(255.0)) as u8 }

fn mult_to_pixel(pixel: &Rgb<u8>, n: f32) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let newr = mult_color_channel(r, n);
    let newg = mult_color_channel(g, n);
    let newb = mult_color_channel(b, n);

    Rgb([ newr, newg, newb ])
}

fn mult_to_r(pixel: &Rgb<u8>, n: f32) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let newr = mult_color_channel(r, n);
    Rgb([ newr, g, b ])
}

fn mult_to_g(pixel: &Rgb<u8>, n: f32) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let newg = mult_color_channel(g, n);
    Rgb([ r, newg, b ])
}

fn mult_to_b(pixel: &Rgb<u8>, n: f32) -> Rgb<u8> {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let newb = mult_color_channel(b, n);
    Rgb([ r, g, newb ])
}
