fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //any variable name like wildcard entry any value is captured and used
    }

    match dice_roll {
        2 => {
            println!("lucky bittt!!");
            add_fancy_hat()
        }
        5 => {
            println!("you supeer lucky");
            remove_fancy_hat()
        }
        _ => {
            //_ does n't give the shit about the value it take any value and ignore it
            println!("shit");
            don_t()
        }
    }
    let dice_rolls = 12;
    match dice_rolls {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn don_t() {}
fn move_player(num_spaces: u8) {}
