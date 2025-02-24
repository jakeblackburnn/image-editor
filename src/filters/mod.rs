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


pub struct FilterFactory {

    filter_constructors: HashMap<String, fn(key_string: &str) -> Box<dyn Filter>>, // maps filter name to function that 
                                                                                   // returns corresponding filter struct
                                                                                   // functions must accept a keystring, even if its empty
}


impl FilterFactory {
    pub fn new() -> Self {
        let mut filter_constructors: HashMap<String, fn(key_string: &str) -> Box<dyn Filter>> = HashMap::new();

        filter_constructors.insert("invert".to_string(), invert::construct);
        filter_constructors.insert("swap".to_string(), swap::construct);
     
        Self {
            filter_constructors,
        }
    }

    pub fn get(&self, filter: &str) -> fn(key_string: &str) -> Box<dyn Filter> {
        self.filter_constructors[filter]
    }
}
