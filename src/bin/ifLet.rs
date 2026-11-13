#![allow(unused)]
enum ServerState {
    Witing,
    Stateless,
    Processing(String),
    Stopped(u32),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let current_state = ServerState::Processing(String::from("server code: 12"));

    if let ServerState::Processing(x) = &current_state {
        //single pattern arm matching using if let
        println!("Server is processing: info: {}", x);
    }
    if let ServerState::Stopped(x) = current_state {
        //single pattern arm matching using if let
        println!("Server is Stopped: info: {}", x);
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
