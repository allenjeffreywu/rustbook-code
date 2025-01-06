// you can run this file with `cargo run --bin if_let`

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    println!("Hello Allen!");

    // we can simplify the following. Basically assignment when it is real via if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // this is prettier, works the same way as match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // this can have an else if there isn't something
    // else is basically _

    let coin = Coin::Quarter(UsState::Alabama);
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }
    // cannot include above because it doesn't return from it... state is a value used after move.

    // this is the same as the above
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
