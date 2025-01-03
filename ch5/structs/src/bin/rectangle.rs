// you can run this file with `cargo run --bin rectangle`

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width1, height1))
    );
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // adding useful functionality with derived traits
    println!("rect1 is {rect1:?}");
    // :? indicates that we want to use an output format called debug
    // when the struct is too big use :#? to print line by line each value

    // another way to debug is to use the dbg! macro - this takes ownership, unlike println! (which takes a reference)

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // debug returns the ownership of the expressions value... so even if it wasn't there width would still be 60

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refactoring with tuples
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// refactoring with structs
// to make this print during debug we MUST have this line above the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
