// enums give you a way of saying a value is one of a possible set of values

// // enums for ip addresses
enum IpAddrKind {
    V4,
    V6,
}

// // a function that takes the above enum
fn route(ip_kind: IpAddrKind) {}

// // we can create a struct with th eenum
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// // set some global variables
// const home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// const loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// we can make the values better
enum IpAddr2 {
    V4(String),
    V6(String),
}

// unlike structs, enums can be all different types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can define methods on enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // create instances of the enum in code
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // this attaches the data to the enum directly

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    // we can also have different amounts of data in the enum
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    // if ran, the self in the enum will be the hello string
    m.call();

    // the Option enum and its advantages over null values
    // when a value could be something or nothing!
    // Rust  does not have the null feature
    // Option<T> is the alternative to null! it is an enum of None and Some(T)
    // <T> is a generic type parameter
    let some_number = Some(5);
    let some_char = Some('e');
    // need to tell compiler we intend for it to be Option<i32>
    let absent_number: Option<i32> = None;

    // this will not compile because we cannot add option to a definite value
    // you must convert Option<T> into T to make it work
    // we can use match to make that happen
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    println!("Hello, world!");
}
