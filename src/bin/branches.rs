use std::io::{self, Write};
fn main() {
    let x: i32 = 12;
    if x <= 10 {
        println!("it is less than 10");
    } else {
        println!("it is greater than 10");
    }
    print!("Enter the number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut user_number = String::new();
    io::stdin().read_line(&mut user_number).expect("error");
    let user_number: i32 = user_number.trim().parse().expect("conversion failed");
    if user_number % 2 == 0 {
        println!("it is even");
    } else if user_number % 2 != 0 {
        println!("it is odd");
    }

    let store = if 5 == 5 { 2 } else { 3 };
    println!("store= {store}");
}
