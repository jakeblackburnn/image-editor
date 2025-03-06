//
// Created By J. Blackburn - Mar 6 2025
//

pub fn get_filter_set(filter_set: String) -> Vec<String> {

    match filter_set.as_str() {
        "basic-swaps" => get_basic_swaps(),

        _ => panic!("couldnt get filter set"),
    }
}

    // keys for keyset identifier: 'basic-swaps'
pub fn get_basic_swaps() -> Vec<String> {

    vec![
        String::from("swap-rgb"),
        String::from("swap-rbg"),
        String::from("swap-grb"),
        String::from("swap-gbr"),
        String::from("swap-brg"),
        String::from("swap-bgr")
    ]

}
