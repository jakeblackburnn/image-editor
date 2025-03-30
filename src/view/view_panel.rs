//
// Created by J. Blackburn - Mar 29 2025
//

use eframe::egui;
use image::{ImageBuffer, Rgb};

use std::sync::{Arc, Mutex};

type RgbImageBuffer    = ImageBuffer<Rgb<u8>, Vec<u8>>;

type SharedImageBuffer = Arc<Mutex<RgbImageBuffer>>;
type SharedBool        = Arc<Mutex<bool>>;

pub struct ViewPanel { 
    image_buffer:  SharedImageBuffer,
    image_texture: Option<egui::TextureHandle>,
    update_switch: SharedBool,
}

impl ViewPanel {

    pub fn new( image_buffer: SharedImageBuffer, update_switch: SharedBool ) -> Self {
        ViewPanel {
            image_buffer:  image_buffer,
            image_texture: None,
            update_switch: update_switch,
        }
    }
    
    fn load_image( ctx: &egui::Context, image_buffer: SharedImageBuffer ) -> egui::TextureHandle {

        let locked_image_buffer = image_buffer.lock()
                                              .unwrap();

        let size = [ locked_image_buffer.width() as usize, locked_image_buffer.height() as usize ];
        let raw_rgba = locked_image_buffer.as_raw();

        let color_image = egui::ColorImage::from_rgb(size, &raw_rgba);

            // return image texture
        ctx.load_texture("image", color_image, Default::default() )
    }

} // end impl ViewPanel
  

impl eframe::App for ViewPanel {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame ) {

            // TODO: Add update functionality

        self.image_texture = Some( ViewPanel::load_image( ctx, self.image_buffer.clone() ));



        egui::CentralPanel::default().show(ctx, |ui| {

            let panel_size = ui.available_size();

            if let Some(texture) = &self.image_texture { // if texture is loaded
                let image_size  = texture.size_vec2();
                let scale       = (panel_size.x / image_size.x).min(panel_size.y / image_size.y);

                let scaled_size = image_size * scale;

                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(scaled_size));
                });
            }
        });
    }

} // end impl eframe App for ViewPanel
