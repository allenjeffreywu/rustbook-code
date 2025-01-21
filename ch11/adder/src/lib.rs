// How to write tests

// bodies of test functions typically perform these three actions:
// 1 set up any needed data or state
// 2 run the code you want to test
// 3 assert that the results are what you expect

// a test is a function in rust that is annotated with the test attribute

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

// run all tests with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed

    /*
    #[test]
    fn another() {
        // panic will make it auto fail
        // panic!("Make this test fail");

        // but we don't need to make it panic
        // we can use assert! macro
    }
     */

    // bring things outside of the test module in, like rectangle
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // asserting a false claim with some boolean logic
        assert!(!smaller.can_hold(&larger));
    }

    // testing equality with the assert_eq! and assert_ne! macros

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    // left = expected
    // right = actual
    // the values being compared must implement the PartialEq and Debug traits
    // you can do #[derive(PartialEq, Debug)] to add the traits to struct or enum definition

    // adding custom failure messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // checking for panics with should_panic
    // be warned, this can be imprecise. it can fail for any other reason to panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    // add expected to make it more precise
    // the expected here is a substring of the message that the panic creates (so we don't have to match EVERYTHING)
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    // using Result<T, E> in tests
    // this allows us to use ? operator in tests (the nullish coalesce)
    // you can't use the #[should_panic] on tests that use Result<T, E>. To assert that an operation returns an Err variant. Use assert!(value.is_err())
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
