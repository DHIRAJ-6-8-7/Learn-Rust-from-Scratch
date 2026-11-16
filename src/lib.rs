mod front_of_house {
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

    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    mod bartender {
        fn make_drink() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    front_of_house::hosting::seat_at_table();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //Rust's compiler looks only inside the current room (back_of_house). It will not magically search upstairs or outside the room on its own. It rows a cannot find function deliver_food in this scope error.
    }

    fn cook_order() {}
}
