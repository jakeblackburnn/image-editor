// 
// Created by J. Blackburn - Jan 19 2025
//

use image::{ImageBuffer, Rgb};
use crate::filters;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub fn start(mut image_buffer: RgbImageBuffer, filter: Option<&String>, save_path: Option<&String>) {

    let factory        = filters::FilterFactory::new();
    let filter_builder = factory.get("invert");

    filter_builder().apply(&mut image_buffer);


        println!("saving image. . .");

    let _  = image_buffer.save("outputs/out.png");

        println!("image saved.");
}   
