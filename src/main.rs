mod binary_search;
mod utils;

use binary_search::{
    search,
    SearchEnum::{
        Found,
        NotFound,
    },
};
use utils::{
    get_i32,
};


fn test_sorted_usize() {
    let mut values: Vec<usize> = vec![
        10, 15, 12, 5, 99, 87, 71, 41, 22, 29,
        3, 31, 84, 52, 63, 81, 58, 92, 49, 77,
    ];
    values.sort();

    for (i, val) in values.iter().enumerate() {
        match search(&values, *val) {
            Found(position, iterations) => {
                println!("{val} found in {:?} in {iterations} iterations", &values);
                assert!(position == i);
            },
            NotFound(_iterations) => {
                println!("Should not happen: searching for {val} in {:?} not found", &values);
            },
        }
    }

    loop {
        let search_for: usize = get_i32("Type number to search for: ");
        match search(&values, search_for) {
            Found(position, iterations) => {
                println!("{search_for} found at position {position} in {:?} after {iterations} iterations", values);
            },
            NotFound(iterations) => {
                println!("{search_for} not found in {:?} after {iterations} iterations", values);
            },
        }
    }
}

fn main() {
    test_sorted_usize();
}
