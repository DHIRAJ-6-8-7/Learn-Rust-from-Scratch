#![allow(unused)]

struct Lamborghini {
    engine: String,
    color: String,
    speed: f32,
    availability: bool,
    seats: u32,
}

struct Buyer {
    name: String,
    age: u32,
    availability: bool,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let dhiraj = Lamborghini {
        engine: String::from("v12"),
        speed: 422.00,
        color: String::from("blue"),
        availability: false,
        seats: 2,
    };

    println!("Engine: {}", dhiraj.engine);
    println!("Color: {}", dhiraj.color);
    println!("Speed: {}", dhiraj.speed);
    println!("Available: {}", dhiraj.availability);
    println!("Seats: {}", dhiraj.seats);

    let mut devaki: Lamborghini = Lamborghini {
        engine: String::from("v16"),
        color: String::from("valvet red"),
        seats: 1,
        availability: true,
        speed: 541.12,
    };

    devaki.seats = 2;

    let buyer1 = make_buyer(String::from("dhiraj"), 20, false);
    println!("Buyer: {}, Age: {}", buyer1.name, buyer1.age);

    let buyer12 = make_buyer(String::from("dhiraj"), 20, false);
    println!("Buyer: {}, Age: {}", buyer12.name, buyer12.age);

    let user1 = User {
        email: String::from("dhiraj@example.com"),
        username: String::from("dhirajshrestha"), //doesn't exist now
        active: true,
        sign_in_count: 1,
    };

    let user2: User = User {
        email: String::from("shresthadhiraj924@gmail.com"),
        active: false,
        ..user1 //ownership of heap allocated is transferred
    };

    println!("user 1: username: {}", user1.active);
    println!("user 2: username: {}", user2.username);
}

fn make_buyer(name: String, age: u32, avail: bool) -> Buyer {
    Buyer {
        name: name,
        age: age,
        availability: avail,
    }
}
fn make_buyer2(name: String, age: u32, availability: bool) -> Buyer {
    Buyer {
        age,
        name,
        availability,
    }
}
