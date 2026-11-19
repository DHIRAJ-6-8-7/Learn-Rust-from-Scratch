#![allow(unused)]
use std::fs::File;

fn main() {
    let unknown_file = File::open("happy.txt");

    let result = match unknown_file {
        Ok(x) => x,
        Err(y) => match y.kind() {
            std::io::ErrorKind::NotFound => match File::create("happy.txt") {
                Ok(next_file) => next_file,
                Err(error) => panic!("problem creating the file: {error:?}"),
            },
            others => panic!("Problem opening the file: {others:?}"),
        },
    };

    let lets_try_file = File::open("andimandisandi.rs").unwrap(); //similar to match operation standalone

    let next_try_file = File::open("andisandi.c").expect("andi mand sandi");
}
