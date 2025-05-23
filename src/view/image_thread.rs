//
// Created by J. Blackburn - Mar 30 2025
//

use crate::filters;

use image::{ImageBuffer, Rgb};

use std::sync::{Arc, Mutex};
use std::thread;
use std::path::Path;
use std::io::{stdin, stdout, Write};



type RgbImageBuffer    = ImageBuffer<Rgb<u8>, Vec<u8>>;
type SharedImageBuffer = Arc<Mutex<RgbImageBuffer>>;
type SharedBool        = Arc<Mutex<bool>>;

#[derive(PartialEq)]
enum MenuOption { // types of possible user inputs
    Empty, 
    Filter, 
    Save, 
    Quit,
}

pub fn start_image_thread( image_buffer: SharedImageBuffer, update_switch: SharedBool ) {

    thread::spawn( move || {

        // TODO: Format txt output

        let mut menu_option = MenuOption::Empty; // default option is none / empty
        
        let factory = filters::FilterFactory::new();


        while menu_option != MenuOption::Quit {

                // TODO: Add proper input menu
            println!("enter option: ");

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read user input");

            let _ = stdout().flush(); // toss junk 

            menu_option = match input.trim() {
                "f" => MenuOption::Filter,
                "s" => MenuOption::Save,
                "q" => MenuOption::Quit,
                 _  => MenuOption::Empty,
            };

                // TODO: implement menu option actions
            match menu_option {

                MenuOption::Filter => {
                    print!("Filtering image - Enter filter identifier: ");
                    let _ = stdout().flush();

                    let mut filter_string = String::new();
                    stdin().read_line(&mut filter_string)
                           .expect("Error: could not read filter identifier");
                    let _ = stdout().flush();

                        // break filter string into name and key
                    let (filter_name, key_string) = 
                        filters::get_filter_components(&filter_string);


                    let filter_builder = factory.get(filter_name);
                    let filter = filter_builder(key_string);

                    let mut locked_image_buffer = image_buffer.lock().unwrap();

                    filter.apply(&mut locked_image_buffer);

                    let mut locked_update_switch = update_switch.lock().unwrap();
                    *locked_update_switch = true;

                    continue;
                }
                MenuOption::Save => {
                    print!("Saving image - Specify output path: ");
                    let _ = stdout().flush();

                    let mut output_path = String::new();
                    stdin().read_line(&mut output_path)
                           .expect("Error: could not read user input");

                    let _ = stdout().flush();

                    let locked_image_buffer = image_buffer.lock().unwrap();
                    locked_image_buffer.save(Path::new(output_path.trim())); 

                    continue;
                }
                MenuOption::Quit => {
                    println!("Quitting");
                    continue;
                }
                MenuOption::Empty => {
                    println!("Option provided is invalid - try again");
                    continue;
                }

            } // end match menu option
        } // end while not quit option
    }); // end spawn thread
}
