//
// Created By J. Blackburn - Jan 20 2025
//

pub mod swap;
pub mod invert;
pub mod plus;
pub mod mult;
pub mod keysets;

use image::{ImageBuffer, Rgb};
use regex::Regex;

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
        filter_constructors.insert("plus".to_string(), plus::construct);
        filter_constructors.insert("mult".to_string(), mult::construct);
     
        Self {
            filter_constructors,
        }
    }

    pub fn get(&self, filter: &str) -> fn(key_string: &str) -> Box<dyn Filter> {
        self.filter_constructors[filter]
    }
}



pub fn get_filter_components(filter_id: &str) -> (&str, &str) {

    let re = Regex::new(r"(?P<name>[a-z]+)-?(?P<key>.*)")
                    .unwrap();

    if let Some(filter_parts) = re.captures(filter_id) {

        let name = filter_parts.name("name").unwrap().as_str();

        let key_string  = match filter_parts.name("key") {

            Some(key) if !key.as_str().is_empty() => key.as_str(),
            _ => "",

        };
    
        (name, key_string)

    } else {
        panic!("Failed to parse filter identifier");
    }

}
