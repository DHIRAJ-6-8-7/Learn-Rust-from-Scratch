#![allow(unused)]
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut opening_file = File::open("unknown.txt")?;
    let mut username = String::new();
    opening_file.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_method2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("unknown.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_easy_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

//The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. Cons
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
