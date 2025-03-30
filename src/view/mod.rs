// 
// Created By J. Blackburn - Jan 20 2025
//

mod view_panel;
use view_panel::ViewPanel;

use crate::utils::load_rgb_image_buffer;


use eframe::egui;
use image::{ImageBuffer, Rgb};

use std::sync::{Arc, Mutex};

type RgbImageBuffer    = ImageBuffer<Rgb<u8>, Vec<u8>>;

type SharedImageBuffer = Arc<Mutex<RgbImageBuffer>>;
type SharedBool        = Arc<Mutex<bool>>;



pub fn start(mut args: std::env::Args) {

        // load base image buffer
    let input_image = args.next().expect("Failed to parse image path");
    let image_buffer = load_rgb_image_buffer(input_image);

        // create shareable clone of image, and update switch
    let mut shared_image_buffer = Arc::new( Mutex::new( image_buffer.clone() ));
    let mut update_switch = Arc::new( Mutex::new( true ));

    let mut app = ViewPanel::new( shared_image_buffer.clone(), update_switch.clone() );

    let _ = eframe::run_native(
        "Image Editor View Panel",
        eframe::NativeOptions::default(),
        Box::new( |_cc| Ok(Box::new(app)))
    );

}
