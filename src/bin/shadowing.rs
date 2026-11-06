#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    let x = 5;

    let x = x + 1;
    //this create the block
    {
        let x = x + 3;
        println!("the value of x: {}", x);
    }
    println!("the value of x: {}", x);

    let result = {
        let x = 5;
        let y = 10;
        x * y
    };

    println!("{result}");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("{spaces}");

    let hexa_num = 0x123fA;
    println!("{:x}", hexa_num);

    let oct_num = 0o12317;
    println!("{:o}", oct_num);

    //# is the flag that add 0 in front
    let bin_num = 0b111011;
    println!("{:#b}", bin_num);

    let x = b'A';
    println!("{x:#b}");
    let addition = 10 + 3;
    let subtraction = 10 - 3;
    let multiplication = 10 * 3;
    let division = 10 / 3;
    let remainder = 10 % 3;

    println!("Addition: {addition}");
    println!("Subtraction: {subtraction}");
    println!("Multiplication: {multiplication}");
    println!("Division: {division}");
    println!("Remainder: {remainder}");

    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let multiple = [5; 100];
    println!("{:?}", multiple);
}
