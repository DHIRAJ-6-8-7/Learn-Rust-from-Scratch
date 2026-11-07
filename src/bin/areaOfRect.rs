#![allow(unused)]
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

//implementing the method of the Rectangle inside the impl block
impl Rectangle {
    //this is associated func not method (Notice: NO `self` parameter)
    //here Self is the alias for the type that impl block is implementing
    fn new(width: u32, height: u32) -> Self {
        //constructor
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn check_fit(&self, rect2: &Rectangle) -> bool {
        if self.area() > rect2.area() {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let x = 20;
    let rect2 = Rectangle {
        width: dbg!(x * 30),
        height: 50,
    };

    println!(
        "The area of the rectangle: {rect1:#?} is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1); //it return the ownership of the evaluated expression

    dbg!(&rect2);

    println!("the are of the rectangle of rect1 is {} ", rect1.area());

    println!("Can rect2 fit in rect1: {}", rect1.check_fit(&rect2));

    let create_object_from_constructor = Rectangle::new(10, 30);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
