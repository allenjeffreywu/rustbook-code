// you can run this file with `cargo run --bin rectangle_class`

// methods are similar to functions. they still use the fn keyword
// however they are defined within the context of a struct
// And their first parameter is always self - python like. Which represents the instance of the struct method is being called on

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl keyword for the struct. basically defines the class
impl Rectangle {
    // very python like
    // this &self is shorthand for self: &Self
    // we can also borrow self mutably like other functions (&mut self). After which the caller cannot use the original instance after transformation

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // there are getter methods but we will talk about them later

    // methods with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions are all functions defined within an impl block
    // we can define an associated function WITHOUT self as the first parameter
    // often used for constructors.

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// multiple impl blocks. each struct is allowed to have multiple!
impl Rectangle {
    fn can_hold2(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// this is valid

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // we can use this associated function with ::
    let rect4 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// notice there is no -> operator
// Rust has automatic referencing and dereferening
// the following are the same:
// p1.distance(&p2);
// (&p1).distance(&p2);
