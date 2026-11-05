fn another_function() {
    println!("This is another function");
    let result: i32 = add_number(10, 12);
    println!("result = {result}");
}
fn main() {
    println!("This is the main function");
    another_function();
}

fn add_number(x: i32, y: i32) -> i32 {
    println!("x = {x}");
    println!("y = {y}");
    print_labeled_measurement(12, '\u{03C0}');
    x + y
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}, {unit_label}");
}
