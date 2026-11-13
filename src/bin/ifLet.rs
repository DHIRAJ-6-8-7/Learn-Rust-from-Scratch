#![allow(unused)]

#[derive(Debug)]
enum ServerState {
    Waiting,
    Stateless,
    Processing(String),
    Stopped(u32),
}

#[derive(Debug)]
enum State {
    Alabama,
    Nepal,
}

enum Coin {
    Nickel,
    Quarter(State),
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let current_state = ServerState::Processing(String::from("server code: 12"));

    if let ServerState::Processing(x) = &current_state {
        // Single pattern matching using if let
        println!("Server is processing: info: {}", x);
    }

    if let ServerState::Stopped(x) = &current_state {
        // Single pattern matching using if let
        println!("Server is stopped: info: {}", x);
    } else {
        println!("bye bye");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

impl State {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            State::Alabama => year >= 1819,
            State::Nepal => year >= 1768,
        }
    }
}

fn get_number(opt: Option<i32>) -> i32 {
    // "Try to unwrap `Some(n)`. If `opt` is actually `None`, run the `else` block."
    let Some(n) = opt else {
        return 0; // The `else` block MUST exit the function (return, panic, or break)
    };

    // If we reached this line, we are on the "Happy Path"!
    // `n` is successfully unpacked and ready to use.
    n * 2
}
