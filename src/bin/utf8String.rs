#![allow(unused)]
// pub struct String {
//     vec: Vec<u8>, // A vector of raw unsigned bytes!
// }
fn main() {
    let mut new_str = String::new();
    let name = "Dhiraj";
    let new_name = name.to_string();
    let another_name = "happy".to_string();
    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");

    s.push_str("Dhiraj");

    println!("{}", s);

    let mut s = String::from("lo");
    s.push('d');
    println!("{}", s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3 + " Happy birthday to you!";

    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1} dhiraj shrestha {s2} happy {s3} birthday to you"); //format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    println!("{}", s);

    let hello = String::from("Здравствуйте");

    let hello = "Здравствуйте";

    let d = &hello[0..8];
    println!("{}", d);

    for i in hello.chars() {
        print!("{i} \t");
    }
    println!("\n");
    for i in hello.bytes() {
        print!("{i} \t");
    }
}
