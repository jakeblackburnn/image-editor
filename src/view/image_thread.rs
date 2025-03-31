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


        while menu_option != MenuOption::Quit {

            // TODO: Add input menu

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

                MenuOption::Filter => {}
                MenuOption::Save => {}
                MenuOption::Quit => {}
                MenuOption::Empty => {}

            } // end match menu option
        } // end while not quit option
    }); // end spawn thread
}
