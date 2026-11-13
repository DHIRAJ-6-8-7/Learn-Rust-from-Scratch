#![allow(unused)]
enum Coins {
    Paisa,
    Rupiya,
    Note,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {}

fn coins_value(coin: Coins) -> u32 {
    match coin {
        Coins::Paisa => 1,
        Coins::Rupiya => 100,
        Coins::Note => {
            println!("hurray!");
            1000
        }
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // comma here is optional for the block

        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
