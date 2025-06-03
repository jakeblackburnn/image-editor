//
// Created By J. Blackburn - Jan 20 2025
//

pub mod keysets; // expose keysets

pub mod mult;
pub mod invert;
pub mod plus;
pub mod swap;
pub mod minus;
pub mod grayscale;
pub mod bw;
pub mod colorize;

    // gather filter structs
use mult::Mult;
use invert::Invert;
use plus::Plus;
use swap::Swap;
use minus::Minus;
use grayscale::Grayscale;
use bw::BlackWhite;
use colorize::Colorize;

use image::{ImageBuffer, Rgb};
use regex::Regex;

use std::fmt;
use std::collections::HashMap;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

    // Filter data structures
type FilterResult  = Result<Box<dyn Filter>, FilterError>;
type FilterBuilder = fn(key: &str) -> FilterResult;
type FilterMap     = HashMap<String, FilterBuilder>;

#[derive(Debug)]
pub enum FilterError {
    InvalidKey,
    FilterNotFound,
    FilterConstructionFailed,
    InvalidFilterFormat,
}
    // make FilterError printable
impl fmt::Display for FilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterError::InvalidKey => write!(f, "Invalid key provided for filter."),
            FilterError::FilterNotFound => write!(f, "The specified filter was not found."),
            FilterError::FilterConstructionFailed => write!(f, "Failed to construct the filter."),
            FilterError::InvalidFilterFormat => write!(f, "The filter string format is invalid."),
        }
    }
}
        // FILTER TRAIT:
    // all filter structures must have an apply method
    // and an associated function returning a result containing the specified filter or an error
pub trait Filter {
    fn apply(&self, image: &mut RgbImageBuffer);
    fn construct(key: &str) -> FilterResult where Self: Sized;
}


        // FILTER FACTORY:
    // holds a mapping of filter names to their respective constructors
    // get method tries to build the filter struct specified, returning FilterResult
pub struct FilterFactory {
    filter_constructors: FilterMap, 
}

impl FilterFactory {
    pub fn new() -> Self {
        let mut filter_constructors: FilterMap = HashMap::new();
        filter_constructors.insert("invert".to_string(), Invert::construct);
        filter_constructors.insert("swap".to_string(), Swap::construct);
        filter_constructors.insert("plus".to_string(), Plus::construct);
        filter_constructors.insert("minus".to_string(), Minus::construct);
        filter_constructors.insert("mult".to_string(), Mult::construct);
        filter_constructors.insert("grayscale".to_string(), Grayscale::construct);
        filter_constructors.insert("colorize".to_string(), Colorize::construct);
        filter_constructors.insert("bw".to_string(), BlackWhite::construct);
     
        Self {
            filter_constructors,
        }
    }

    pub fn get(&self, filter_str: &str) -> FilterResult {
        let (name, key) = match get_filter_components(filter_str) {
            Some((n,k)) => (n,k),
            None        => return Err(FilterError::InvalidFilterFormat),
        };
        match self.filter_constructors.get(name) {
            Some(builder) => builder(key),
            None          => Err(FilterError::FilterNotFound),
        }
    }
}

fn get_filter_components(filter_str: &str) -> Option<(&str, &str)> {
    let re = Regex::new(r"(?P<name>[a-z]+)-?(?P<key>.*)")
                    .unwrap();

    if let Some(c) = re.captures(filter_str) {

        let name_str = match c.name("name") {
            Some(name) if !name.as_str().is_empty() => name.as_str(),
            _ => return None,
        };

        let key_str  = match c.name("key") {
            Some(key) if !key.as_str().is_empty() => key.as_str(),
            _ => "",
        };

        Some((name_str, key_str))
    } else {
        None
    }
}
