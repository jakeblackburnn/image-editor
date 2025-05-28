//
// Created by J. Blackburn - Mar 5 2025
//

use crate::utils::load_rgb_image_buffer;

use crate::filters;


pub fn single_run(input_image: String, filter_string: String, output_path: String) {

    let mut image_buffer = load_rgb_image_buffer(input_image);

        // break filter string into name and key
    let Some((filter_name, key_string)) = filters::get_filter_components(&filter_string) 
    else {
        panic!("failed to get filter components");
    }; 

        // build filter via filter factory
    let factory = filters::FilterFactory::new();
    let filter_builder = factory.get(filter_name);
    let filter = filter_builder(key_string);

    

    filter.apply(&mut image_buffer);

    let _ = image_buffer.save(output_path);

}


pub fn batch_output_run(input_image: String, filter_set_string: String, output_dir: String) {
    
    let image_buffer = load_rgb_image_buffer(input_image);

    let filter_set = filters::keysets::get_filter_set(filter_set_string);

    let factory = filters::FilterFactory::new();

    for filter_string in filter_set {

            // create mutable clone of base image
        let mut image_buffer_clone = image_buffer.clone();

            // break filter string into name and key
        let Some((filter_name, key_string)) = filters::get_filter_components(&filter_string)
        else {
            panic!("failed to get filter components");
        }; 

            // build filter via filter factory
        let filter_builder = factory.get(filter_name);
        let filter = filter_builder(key_string);



        filter.apply(&mut image_buffer_clone);

            // create output path string, default is output_dir/filter.png
        let output_path = format!("{}/{}.png", output_dir, filter_string);

        let _ = image_buffer_clone.save(output_path);
    }
}
