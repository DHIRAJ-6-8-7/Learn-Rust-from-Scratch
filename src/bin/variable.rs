#![allow(dead_code)] // this rust attribute the instruction to the compiler directly
//my code contain experimental unused value so i am using it
static VERSION: &str = "1.0.0.1"; //true global variable
const PATCH: &str = "81.141.4"; // the inliner
fn main() {
    //variable
    let x: i32 = 20;
    let x = x * 10;
    let y: u8 = 10;
    let letter: char = 'N';
    let b: f64 = 30.2;
    let c: f64 = x as f64;
    let flag: bool = true;
    let money = 1_000_000;
    let unknown = 32.234f64;
    //shadow masking
    let name: &str = "dhiraj";
    let mut _name: &str = "  ";

    //tuple
    let _group1: (&str, u8, bool) = ("dhiraj shrestha", 18, true);
    let _group2 = ("dhiraj shrestha", 18, true);

    //array
    let _arr1: [i32; 6] = [10, 0, 3, 6, 2, 6];
    let _arr2 = [10, 0, 3, 6, 2, 6];

    //constant

    println!("{x},{y},{letter},{b},{c},{flag},{money},{unknown},{name},{_name}");
    println!("{}", _arr1[2]);
    println!("{}", _group1.1);

    //casting

    let gas_price: f32 = 100.1;
    let fuel_tank: u8 = 20;
    let total_price: f32 = fuel_tank as f32 * gas_price;
    println!("total_price: {:.3}", total_price);
}
