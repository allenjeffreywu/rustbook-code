// example struct, has fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// using TUPLE STRUCTS without named fields to create different types
// they have the added meaning the struct name provides but don't have names associted with their fields.
// useful when you want to give the whole tuple a name and make the tuple a different type from other tuples

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit like structs without any fields
// these are structs without any fields.
//useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself (thanks inheritance)
struct AlwaysEqual;

// this is the normal way without the shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// this is the good way with the shorthand
// works because the parameter names and the struct field names are exactly the same
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // of course similar to structs elsewhere
    // similar to tuple as well, but you can name each piece of data
    println!("Hello, world!");

    // instatiation, does not have to be in order
    // needs key value pairs
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // to access values use dot notation
    user1.email = String::from("anotheremail@example.com");

    // creating instances from other instances with struct update syntax
    // .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

    let mut user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // .. MUST come last

    // is user2 a deep copy of user1? let's change a value.
    user1.username = String::from("allenwu");

    println!("Username now is: {0}", user2.username);
    // confirmed that user2 is a deep copy of user1. Changing a value in user1 is not a change in value for user2

    // usage of a tuple struct. These have the same fields in them, but are different structs entirely, and not equivalent!
    let black = Color(0, 5, 0);
    let origin = Point(0, 0, 0);

    // getting an individual value from the tuple
    // you can also destructure them like a tuple https://users.rust-lang.org/t/how-to-destructure-a-tuple-struct/45296
    println!("black value at 1 is: {0}", black.1);

    // unitlike struct
    let subject = AlwaysEqual;

    // ownership of struct data
    // we used String type instead of &str because we want the struct to own the data and be valid for as long as possible. We do not want lifetime errors. Will discuss how to solve issues relating to this in ch10
}
