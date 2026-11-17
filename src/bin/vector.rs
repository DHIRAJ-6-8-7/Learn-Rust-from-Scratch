#![allow(unused)]

enum ToStoreMultipleValueInVector {
    Index(u32),
    Name(String),
    Age(u32),
    Completed(bool),
}

fn main() {
    let mut vec: Vec<u32> = Vec::new();

    let initialized_vec = vec!["dhiraj", "shrestha"];
    let mut initialized_vec_string = vec![String::from("Dhiraj"), String::from("shrestha")];

    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("{:?}", vec);

    let v = vec![1, 2, 3, 4, 5, 6];
    let hey = v[3];
    let mut fourth: &i32 = &v[3];

    println!("The fourth element is {fourth}");

    let fourth: Option<&i32> = v.get(3);

    match fourth {
        Some(n) => {
            println!("the fourth element is {n}")
        }
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    v.push(6); // it involve mutable reference Borrow checker rule

    println!("The first element is: {:?}", v);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 12;

        print!("{i} \t");
        println!("{i:p}");
    }

    let mut storing_different_type_in_vector: Vec<ToStoreMultipleValueInVector> = vec![
        ToStoreMultipleValueInVector::Index(30),
        ToStoreMultipleValueInVector::Name(String::from("Dhiraj Shrestha")),
        ToStoreMultipleValueInVector::Age(20),
        ToStoreMultipleValueInVector::Completed(true),
    ];

    for i in &storing_different_type_in_vector {
        match i {
            ToStoreMultipleValueInVector::Index(n) => println!("Index: {}", n),
            ToStoreMultipleValueInVector::Name(n) => println!("Name: {}", n),
            ToStoreMultipleValueInVector::Age(n) => println!("Age: {}", n),
            ToStoreMultipleValueInVector::Completed(n) => println!("Completed: {}", n),
        }
    }
}
