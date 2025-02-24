// 
// Created by J. Blackburn - Jan 19 2025
//

use image::{ImageBuffer, Rgb};
use regex::Regex;

use crate::filters;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub fn start(mut image_buffer: RgbImageBuffer, filter_arg: Option<&String>, save_path_arg: Option<&String>) {

        // For now these arguments are required
    let filter_id      =     filter_arg.map(String::as_str).expect("no filter specified");
    let save_path      =  save_path_arg.map(String::as_str).expect("no save path specified");




    let (filter_name, key_string) = get_filter_components(filter_id);

            // Get specified filter from factory
        let factory        = filters::FilterFactory::new();

        let filter_builder = factory.get(filter_name);

        let filter = filter_builder(key_string);

            // Apply dynamically loaded filter to image buffer
        filter.apply(&mut image_buffer);



            // Save the modified image

            println!("saving image. . .");

        let _  = image_buffer.save(save_path);

            println!("image saved.");



}   



fn get_filter_components(filter_id: &str) -> (&str, &str) {

    let re = Regex::new(r"(?P<name>[a-z]+)-(?P<key>.*)")
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
