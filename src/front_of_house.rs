pub enum Appetizer {
    Soup,
    Salad,
}
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn choose_bread(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("chocolate"),
        }
    }
}

pub fn weclome() {}

pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

mod bartender {
    fn make_drink() {}
}
