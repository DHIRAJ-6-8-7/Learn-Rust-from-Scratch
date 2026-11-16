#![allow(unused)]

use std::fmt::Result;
use std::fs::File;

use std::io::{Result as IoResult, Write};

use std::collections::*;
use std::io::{self, Read}; //  * this this glob operator any item inside the module define public is all imported

// Returns std::fmt::Result
fn function1() -> Result {
    // fmt::Result is an alias for:
    // std::result::Result<(), std::fmt::Error>

    Ok(())
}

// Returns std::io::Result
fn function2() -> IoResult<()> {
    // Create a file.
    let mut file = File::create("example.txt")?;

    // Write content into the file.
    file.write_all(b"Hello from Rust!")?;

    Ok(())
}

fn main() {
    match function1() {
        Ok(()) => println!("Function 1 completed successfully"),
        Err(error) => println!("Formatting error: {error}"),
    }

    match function2() {
        Ok(()) => println!("Function 2 completed successfully"),
        Err(error) => println!("I/O error: {error}"),
    }
}

mod front_of_house;
pub use crate::front_of_house::hosting; //Bring this item into my current scope, and make it public so that anyone using my library can access it right from here.

#[test]
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = front_of_house::Breakfast::choose_bread("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment i t; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // absolute path
    hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::seat_at_table();

    let order1 = front_of_house::Appetizer::Soup;
    let order2 = front_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
