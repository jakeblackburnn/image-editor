//
// Created by J. Blackburn - Mar 30 2025
//

use crate::filters::FilterFactory;

use image::{ImageBuffer, Rgb};

use std::sync::{Arc, Mutex};
use std::thread;
use std::path::Path;
use std::io::{stdin, stdout, Write};

type RgbImageBuffer    = ImageBuffer<Rgb<u8>, Vec<u8>>;
type SharedImageBuffer = Arc<Mutex<RgbImageBuffer>>;
type SharedBool        = Arc<Mutex<bool>>;



pub fn start_image_thread( image_buffer: SharedImageBuffer, update_switch: SharedBool ) {
    thread::spawn( move || {

        // TODO: Format txt output
        loop {
            print!("enter option: ");
            let _ = stdout().flush(); 

            let input = read_input();
            match input.as_str() {
                "f" => {
                    handle_filter_option( image_buffer.clone(), update_switch.clone() );
                    continue;
                },
                "s" => {
                    handle_save_option( image_buffer.clone() );
                    continue;
                },
                "q" => {
                    break;
                },
                 _  => {
                    println!("Option provided is invalid - try again");
                    continue;
                },
            };
        }
    }); // end spawn thread
}



fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read user input");
    let _ = stdout().flush(); 

    input.trim().to_string()
}



fn handle_filter_option(image_buffer: SharedImageBuffer, update_switch: SharedBool) {
    print!("Filtering image - Enter filter identifier: ");

    loop {
        let _ = stdout().flush();

        let filter_string = read_input();
        let factory = FilterFactory::new();

        match factory.get(&filter_string) {
            Ok(filter) => {
                let mut locked_image_buffer = image_buffer.lock().unwrap();
                filter.apply(&mut locked_image_buffer);
                let mut locked_update_switch = update_switch.lock().unwrap();
                *locked_update_switch = true;

                break;
            }
            Err(e) => { 
                println!("Error: {}", e);
                print!("try again: ");
                continue;
            }
        };
    }; // end loop
}



fn handle_save_option(image_buffer: SharedImageBuffer) {
    print!("Saving image - Specify output path: ");
    let _ = stdout().flush();

    let output_path = read_input();

    let locked_image_buffer = image_buffer.lock().unwrap();
    locked_image_buffer.save(Path::new(&output_path)); 
}
