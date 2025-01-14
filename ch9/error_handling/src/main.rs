use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};
// two types of errors
// 1. recoverable - report the problem to the user and retry the operation
// 2. unrecoverable - reaching a location beyond the end of an array, so we want to stop the program.

// remember, Rust doesn't have exceptions. It has type Result<T, E> for recoverable errors
// and the panic! macro when the program reaches an unrecoverable error

fn main() {
    // this is temporary like all other things
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");
    // unrecoverable Errors with panic!
    // panic will print a failure message, unwind, clean up the stack, and quit
    // you can have rust display the call stack when a panic occurs to make it easier to track down (done via environment variable)

    // also with an environment variable, you can force Rust to not unwind and abort instead via panic = 'abort' in the Cargo.toml

    // panic!("crash and burn");

    // access outside an array
    // will tell us exactly what happened
    /*
    let v = vec![1, 2, 3];

    v[99];
     */

    // Recoverable errors with Result

    // result looks like the following:
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */

    // T represents a type of the value that will be returned in a success case within the Ok variant
    // E represents the type of the error that will be returned in a failure case within the error variant

    // something that can fail
    let greeting_file_result = File::open("hello.txt");

    // reminder that Result is included in the prelude, so Result:: is not required
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    // this is really cool. no try catch finally

    // there are alternatives to using match
    // you can unuwrap_or_else
    /*
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
     */
    // does the same thing but more concise

    // unwrap method
    // If ok variant, unwrap will return the value inside the Ok
    // If err variant, unwrap will call the panic macro
    let greeting_file = File::open("hello.txt").unwrap();

    // expect method lets us choose the panic error message. Used instead of unwrap
    // be more specific with error messages
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // prefer expect over unwrap to be more precise

    // propagating errors
    // instead of error handling inside the function, you can give it to the caller.
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        // this returns, so a ; is not needed
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // shortening the function with ?
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // ? placed after a result value is defined to work in the same way as the match expressions
    // when the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function

    // just like typescript
    // option chaining!
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // even shorter using built in functions
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // Where the ? operator can be used
    // it can only be used in a function that returns Result, Option, or another type that implements FromResidual.
    // Because main returns (), we cannot use ? here
    // can be used with Option with None and Some (in the same way that Err and Ok work)
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    // ? operator will not convert a Result to an Option and vice versa

    // main function may return any types that implement the std::process:Termination trait

    // To panic1 or not to panic!

    // if a method call fails in a test, you'd want the whole test to fail.
    // panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen

    // cases in which you have more information than the compiler - just unwrap or expect
    // because chances are if it does fail even if you know it shouldn't - you would know why because you wrote a message

    // Guidelines for error handlinng
    // panic! when your code could end up in a bad state (when an assumption, guarantee, contract, or invariant has been broken)
    // when a failure is expected, return a Result
    // DO NOT OPERATE ON POTENTIALLY INVALID DATA

    // Creating Custom Types for Validation
}
pub struct Guess {
    value: i32,
}

// always have to create a new guess here, and therefore it will ALWAYS be checked!
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
