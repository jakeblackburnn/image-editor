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

        let add_mode  = String::from("add");
        let view_mode = String::from("view");

        match mode {
            add_mode  =>  add::start(args_iterator),
          //view_mode => view::start(args_iterator),    ... gonna ignore view mode for now

            _      => panic!("Mode argument invalid"),
        }

    } else {
        panic!("Mode argument not found");
    }
}
