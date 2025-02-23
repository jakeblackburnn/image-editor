// 
// Created by J. Blackburn - Jan 19 2025
//

use image::{ImageBuffer, Rgb};
use crate::filters;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub fn start(mut image_buffer: RgbImageBuffer, filter_arg: Option<&String>, save_path_arg: Option<&String>) {

        // For now these arguments are required
    let filter_name    =     filter_arg.map(String::as_str).expect("no filter specified");
    let save_path      =  save_path_arg.map(String::as_str).expect("no save path specified");


        // Get specified filter from factory
    let factory        = filters::FilterFactory::new();
    let filter_builder = factory.get(filter_name);
    let filter = filter_builder();

        // Apply dynamically loaded filter to image buffer
    filter.apply(&mut image_buffer);



        // Save the modified image

        println!("saving image. . .");

    let _  = image_buffer.save(save_path);

        println!("image saved.");
}   
