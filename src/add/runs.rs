//
// Created by J. Blackburn - Mar 5 2025
//

use crate::utils::load_rgb_image_buffer;

use crate::filters;


pub fn single_run(input_image: String, filter: String, output_path: String) {

    let mut image_buffer = load_rgb_image_buffer(input_image);

        // break filter identifier into name and key
    let (filter_name, key_string) = filters::get_filter_components(&filter);

        // build filter via filter factory
    let factory = filters::FilterFactory::new();
    let filter_builder = factory.get(filter_name);
    let filter = filter_builder(key_string);

    

    filter.apply(&mut image_buffer);

    let _ = image_buffer.save(output_path);

}
