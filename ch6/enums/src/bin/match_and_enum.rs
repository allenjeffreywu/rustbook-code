// you can run this file with `cargo run --bin match_and_enum`

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // think of it like a switch case
    // can use curly braces if there is more advanced logic
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            // still returns the last expression here
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    // showing that we can handle values as well!
    let coin1 = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin1);

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    // Some is a special word, it is any. opposite of None
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matches are exhaustive
    // the arm's patterns MUST cover all possibilities
    // if we remove the None case from the plus_one fn, then we get a compile error because it doesn't know how to handle the none case

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // Catch-all Patterns and the _ Placeholder

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // every other case handled by other - I can change to whatever name
        other1 => move_player(other1),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    // why does this work? because the last pattern will match all values not specifically listed..
    // rust will warn if you but arms after the catch all arm

    // pattern for when we don't want to use the value in the catch all arm and that is _ (this is just like python) so rust won't warn us about an unused variable
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // this is the very special do nothing
        _ => (),
    }
}
