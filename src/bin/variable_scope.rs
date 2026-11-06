#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
fn main() {
    // s is not valid here, since it's not yet declare
    let s: &str = "Dhiraj Shrestha"; // s is valid from this point forward

    let mut str: String = String::from("Dhiraj Shrestha");
    str.push_str(" is the handsome boy");
    println!("{str}");

    let copying: String = String::from("this is original string"); //this is not available
    let copying2 = copying;
    println!("{copying2}");

    //move example i.e., shallow copy but prev var get invalided
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    //deep copy using clone()
    let s = String::from("humpty dumpty");
    let s2 = s.clone();
    println!("{s}\n {s2}");
} // this scope is now over, and s is no longer valid
