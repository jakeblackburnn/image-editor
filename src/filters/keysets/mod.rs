//
// Created By J. Blackburn - Mar 6 2025
//

pub fn get_filter_set(filter_set: String) -> Vec<String> {

    match filter_set.as_str() {
        "basic-swaps" => get_basic_swaps(),
        "all-swaps" => get_all_swaps(),

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

    // keys for keyset identifier: 'all-swaps'
pub fn get_all_swaps() -> Vec<String> {

    vec![
        String::from("swap-rgb"),
        String::from("swap-rbg"),
        String::from("swap-grb"),
        String::from("swap-gbr"),
        String::from("swap-brg"),
        String::from("swap-bgr"),

        String::from("swap-rrr"),

        String::from("swap-rrg"),
        String::from("swap-rgr"),
        String::from("swap-grr"),

        String::from("swap-rrb"),
        String::from("swap-rbr"),
        String::from("swap-brr"),

        String::from("swap-ggg"),

        String::from("swap-ggr"),
        String::from("swap-grg"),
        String::from("swap-rgg"),

        String::from("swap-ggb"),
        String::from("swap-gbg"),
        String::from("swap-bgg"),

        String::from("swap-bbb"),

        String::from("swap-bbr"),
        String::from("swap-brb"),
        String::from("swap-rbb"),

        String::from("swap-bbg"),
        String::from("swap-bgb"),
        String::from("swap-gbb"),
    ]

}
