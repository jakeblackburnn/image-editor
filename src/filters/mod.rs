//
// Created By J. Blackburn - Jan 20 2025
//

pub mod swap;
pub mod invert;

use image::{ImageBuffer, Rgb};
use std::collections::HashMap;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub trait Filter {
    fn apply(&self, image: &mut RgbImageBuffer);
}


struct FilterFactory {

    filter_constructors: HashMap<String, fn() -> Box<dyn Filter>>, // maps filter name to function that 
                                                                   // returns corresponding filter struct
}


impl FilterFactory {
    fn new() -> Self {
        let mut filter_constructors: HashMap<String, fn() -> Box<dyn Filter>> = HashMap::new();

        filter_constructors.insert("invert".to_string(), invert::construct);
     
        Self {
            filter_constructors,
        }
    }
}
