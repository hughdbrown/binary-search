mod binary_search;
mod customer;
mod utils;

use binary_search::{
    search,
    SearchEnum::{
        Found,
        NotFound,
    },
};
use customer::{
    Customer,
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
        match search(&values, val) {
            Found(position, iterations) => {
                println!("{val} found in {:?} in {iterations} iterations", &values);
                assert!(position == i);
            },
            NotFound(_iterations) => {
                println!("Should not happen: searching for {val} in {:?} not found", &values);
            },
        }
    }

    for _ in 0..3 {
        let search_for: usize = get_i32("Type number to search for: ");
        match search(&values, &search_for) {
            Found(position, iterations) => {
                println!("{search_for} found at position {position} in {:#?} after {iterations} iterations", values);
            },
            NotFound(iterations) => {
                println!("{search_for} not found in {:#?} after {iterations} iterations", values);
            },
        }
    }
}

fn test_sorted_customers()
{
    let mut customers: Vec<Customer> = Vec::new();
    {
        let num_purchases: Vec<usize> = vec![
            10, 15, 12, 5, 99, 87, 71, 41, 22, 29,
            3, 31, 84, 52, 63, 81, 58, 92, 49, 77,
        ];
        for (i, np) in num_purchases.iter().enumerate() {
            let c: Customer = Customer::new(i, *np);
            customers.push(c);
        }
    }
    customers.sort();
    println!("Customers: {:#?}", customers);

    for (i, customer) in customers.iter().enumerate() {
        match search(&customers, customer) {
            Found(position, iterations) => {
                println!("{customer} found in {iterations} iterations");
                assert!(position == i);
            },
            NotFound(_iterations) => {
                println!("Should not happen: searching for {customer} in {:#?} not found", &customers);
            },
        }
    }

    loop {
        let search_for: usize = get_i32("Type number of purchases to search for: ");
        let c: Customer = Customer::new(0, search_for);
        match search(&customers, &c) {
            Found(position, iterations) => {
                println!("num_purchases=={search_for} found at position {position} after {iterations} iterations");
            },
            NotFound(iterations) => {
                println!("num_purchases=={search_for} not found in {:#?} after {iterations} iterations", customers);
            },
        }
    }
}

fn main() {
    test_sorted_usize();
    test_sorted_customers();
}
