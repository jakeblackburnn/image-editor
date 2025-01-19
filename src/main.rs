//
// Created by J. Blackburn - Jan 11 2025
//
 
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {

        Some("add") => {
            println!("add");
        }

        Some("view") => {
            println!("view");
        }

        _ => {
            println!("mode argument invalid");
        }
    }

    println!("theres nothing here yet.");
}
