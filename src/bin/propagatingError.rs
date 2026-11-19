#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn read_use_from_file() -> Result<String, io::Error> {
    let file_user_name = File::open("unknown.txt");

    let mut result = match file_user_name {
        Ok(x) => x,
        Err(y) => return Err(y),
    };

    let mut username = String::new();
    match result.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(u) => Err(u),
    }
}
fn main() {
    let result = read_use_from_file();
    match result {
        Ok(x) => println!("{}", x),
        Err(y) => panic!("panicked"),
    }
}
