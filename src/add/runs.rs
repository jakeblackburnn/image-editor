//
// Created by J. Blackburn - Mar 5 2025
//

use crate::utils::load_rgb_image_buffer;
use crate::filters;

pub fn single_run(input_image: String, filter_string: String, output_path: String) {
    let mut image_buffer = load_rgb_image_buffer(input_image);
    let factory = filters::FilterFactory::new();

    match factory.get(&filter_string) {
            Ok(f) => f.apply(&mut image_buffer),
            Err(_) => panic!("Couldnt build that filter!"),
        };

    let _ = image_buffer.save(output_path);
}



pub fn batch_output_run(input_image: String, filter_set_string: String, output_dir: String) {
    let image_buffer = load_rgb_image_buffer(input_image);

    let factory = filters::FilterFactory::new();

    let filter_set = filters::keysets::get_filter_set(filter_set_string);
    for filter_string in filter_set {
        let mut image_buffer_clone = image_buffer.clone();

        match factory.get(&filter_string) {
            Ok(f) => f.apply(&mut image_buffer_clone),
            Err(_) => panic!("Couldnt get filter!"),
        }
        let output_path = format!("{}/{}.png", output_dir, filter_string);
        let _ = image_buffer_clone.save(output_path);
    }
}
