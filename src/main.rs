//
// Created by J. Blackburn - Jan 11 2025
//
 
mod add;
mod view;
mod filters;
mod utils;

use std::env;

// IMAGE EDITOR - MAIN

fn main() {

    let mut args_iterator = env::args(); // Implements Iterator
    let _ = args_iterator.next();        // toss program executable arg
                                        

    if let Some(mode) = args_iterator.next() {

        match mode.as_str() {
            "add"  =>  add::start(args_iterator),
            "view" => view::start(args_iterator),    

            _      => panic!("Mode argument invalid"),
        }

    } else {
        panic!("Mode argument not found");
    }
}


