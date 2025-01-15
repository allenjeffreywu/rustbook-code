// parameter list is any concrete slice of i32 values we might pass into the function
// returns a reference to the largest value in the list
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// we place type name declarations inside angle brackets <> between the name of the function and the parameter list
fn largest_2<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // currently doesn't compile without PartialOrd
        if item > largest {
            largest = item;
        }
    }

    largest
}

// in struct definitions
struct Point<T> {
    x: T,
    y: T,
}

// to define a struct where there can be multiple generics of different types you can do the following:
struct Point_2<T, U> {
    x: T,
    y: U,
}

// In enums
/*
enum Option<T> {
    Some(T),
    None,
}
*/

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

// In method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// can also specify constraints on generic types to be more specific
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point_3<X1, Y1> {
    x: X1,
    y: Y1,
}

// we can be slightly more specific with generics
impl<X1, Y1> Point_3<X1, Y1> {
    // we declare that the other is generic before and in the method parameters
    fn mixup<X2, Y2>(self, other: Point_3<X2, Y2>) -> Point_3<X1, Y2> {
        Point_3 {
            x: self.x,
            y: other.y,
        }
    }
}

// performance of code using generics
// not slower than with concrete types. Why? Rust does monomorphization of the code by using generics at compile time
// monomorphization = turning generic code into specific code by filling in the concrete types that re used when compiled
// Though this increases the size of the executable...

fn main() {
    println!("Hello, world!");
    // removing duplication by extracting a function
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
    // extract it to a function

    // generic data types
    // can be used for structs
    // they both have to be the same type though
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point_2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Point_3 { x: 5, y: 10.4 };
    let p2 = Point_3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
