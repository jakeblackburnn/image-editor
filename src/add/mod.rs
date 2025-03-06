// 
// Created by J. Blackburn - Jan 19 2025
//

use image::{ImageBuffer, Rgb};
use regex::Regex;

use std::env::args;

use crate::filters;

mod runs;

type RgbImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;



pub fn start(mut args: std::env::Args) {


    let mut batch_input  = false;
    let mut batch_output = false;

    let mut input = args.next().expect("Failed to parse input");
    if input.as_str() == "-b" {
        batch_input = true;
        input = args.next().expect("Failed to parse input");
    }

    let mut key_identifier = args.next().expect("Failed to parse filter key");
    if key_identifier.as_str() == "-s" {
        batch_output = true;
        key_identifier = args.next().expect("Failed to parse filter key");
    }

    if batch_input && batch_output { panic!("Cant do batch input and output yet sorry."); }

    let mut output = args.next().expect("Failed to parse output");

    if batch_input {
        batch_input_run(input, key_identifier, output);
        return;
    }

    if batch_output {
        batch_output_run(input, key_identifier, output);
        return
    }

    runs::single_run(input, key_identifier, output);

}


fn batch_input_run(input_dir: String, filter_key: String, output_dir: String) {
    println!("batch input run");
}

fn batch_output_run(input: String, keyset: String, output_dir: String) {
    println!("batch output run");
}
