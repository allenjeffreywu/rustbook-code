// common imports
use std::cmp::Ordering;
use std::io;
// we added this to the Cargo.toml, which also generated Cargo.lock
// if we wanted to update the packages we could do so with cargo update
// if you want to see documentation surrounding the crates mentioned use cargo doc -- open
use rand::Rng;

fn main() {
    // remember that these are macros, not functions, to print. Will explore more later
    println!("Guess the number!");

    // this is not a spread operator
    // the = here means it is inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // creating a variable
        // declare mut so it is mutatable, variables are immutable by default
        // declare it is of String type
        let mut guess = String::new();

        // reading in with the standard library function
        // take the next line immediately
        // & here indicates that we are referencing guess. unlike other languages you have to declare it mutable instead of it being implied.
        // the expect handles failure from read_line. read_line returns a result type that can fail
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we need to cast guess from a string into an integer that can be processed by cmp
        // this even has error handling. This may get a little annoying.
        // _ is a catch all value. sorta like e
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the {} here is a placeholder. Unlike other languages you do not have to bind the variables to get these to work
        // println!("You guessed: {}", guess);
        // e.g. This will also work
        println!("You guessed: {guess}");

        // match expression is made up of arms, sort of like a switch case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
