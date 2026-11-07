use std::any::Any;

struct Nothing; //this is unit like struct
fn main() {
    let everything = Nothing;
    println!("{:#?}", everything.type_id());
}
