#![allow(unused)]
use std::collections::HashMap;
fn main() {
    let mut new_map: HashMap<String, u32> = HashMap::new();

    new_map.insert(String::from("Dhiraj Shrestha"), 30);
    new_map.insert(String::from("Raunak Bikram Shahi"), 32);

    let top_scorer = new_map.get("Dhiraj Shrestha");

    match top_scorer {
        Some(n) => println!("{}", n),
        None => println!("nothing"),
    }

    let second_scorer = new_map.get("Raunak Bikram Shahi").copied().unwrap_or(10);
    println!("{}", second_scorer);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &new_map {
        println!("{key}: {value}");
    }

    println!("{:?}", new_map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
