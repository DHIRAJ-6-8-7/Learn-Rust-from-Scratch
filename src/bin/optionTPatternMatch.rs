fn add_value_if_exist(value: Option<u32>) -> Option<u32> {
    match value {
        Some(n) => Some(n + 1),
        None => None,
    }
}
fn main(){
    let five = Some(5);
    let if_added = add_value_if_exist(five);
    println!("{:?}", if_added);

}
