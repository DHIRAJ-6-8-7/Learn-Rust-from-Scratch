#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    let s1: String = String::from("dhiraj Shrestha");
    let len = calc_length(&s1);
    println!("length is {}", len);
    println!("length is {}", calc_length(&s1));

    //mutable reference
    //You can have any number of immutable references (&T) OR one mutable reference (&mut T). You cannot mix them.
    let mut str = String::from("pranoide ");
    modify_string(&mut str);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    let mut s = String::from("hello");

    let r1 = &mut s; // r1 starts here
    r1.push_str(" world");
    // ----> This is the LAST time r1 is ever used!
    // Therefore, r1's scope/lifetime ends right here.

    let r2 = &mut s; // ✅ This compiles! Even though we are still in `main`,
    // r1 is already dead, so r2 is the only active mutable reference.
    r2.push_str("!");

    println!("{s}"); // Prints: hello world!
}
fn calc_length(incoming_str: &str) -> usize {
    // s is a reference to a String
    incoming_str.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped

fn modify_string(str: &mut String) {
    str.push_str(", handsome boy");
}
