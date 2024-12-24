fn main() {
    // const, so it wont change
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    // wasn't mut, so error
    x = 6;
    println!("The value of x is: {x}");
    shadowing();
    datatypes();
    compoundtypes();
    another_function(5);
    statements_and_expressions();

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
    control_flow();
}

fn shadowing() {
    let x = 5;

    // creating a new variable with the same name as the old variable is shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // this is valid
    let spaces = "   ";
    let spaces = spaces.len();

    // but when you do the following it is illegal because it has a declared type:
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
     */
}

fn datatypes() {
    // rust is statically typed

    /*
       Scalar Types
       1. integers
       2. floating-point numbers
       3. booleans
       4. characters
    */

    // integers behave like c++ nothing new here
    // Just like c++ you can have different representations
    // decimal
    // hex 0xff
    // octal 0o77
    // binary 0b111
    // byte (u8 only) b'A'
    // byte is so good that it can check for integer overflows
    // with release flag it does two's complement wrapping. You can handle this behavior with methods in the standard library

    // floating point behaves like c++ f32 f64. default is f64.

    // numerics
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean
    // two values, one byte in size
    let t = true;

    let f: bool = false; // with explicit type annotation

    // character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // unicode
    // char is 4 bytes. so worry about utf8
}

fn compoundtypes() {
    // tuple - general way of grouping together a number of values with a variety of types into one compound type.
    // Have fixed length, once declared they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // can also access y as x.1

    // array type
    // fixed length
    // arraylist equivalent is vector (just like c++)
    let a = [1, 2, 3, 4, 5];

    // can specify type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // can specify repeated values easily
    let a = [3; 5];

    // access arrays just as you normally would
}

// I've been doing it wrong throughout this file, but the standard is snake case for functions.
// can define parameters. can have multiple separated by commas
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn statements_and_expressions() {
    // statements are instructions that perform some action and do not return a value
    // expressions evaluate to a resultant value
    let y = 6;
    // below will not compile because let does not return!
    // let x = (let y = 6);

    // instead do the following
    // this block returns so it works.
    // calling a function is an expression
    // calling a macro is an expression
    // new scope block created with curly brackets is an expression.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// return values are not defined before the function name.
// They are defined just like python, by arrows after the function.
fn five() -> i32 {
    5
}

// this is an expression because it does NOT have a ;
// out a ; in there and it breaks.
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // strange ternary...
    let number = if condition { 5 } else { 6 };

    // this fails
    // let number = if condition { 5 } else { "six" };
    // because mismatched types.

    println!("The value of number is: {number}");

    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // all loop logic is identical to other languages.
    // breaks and continue works for innermost loops

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for each loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for loop with iterator
    // rev to reverse the range. very python like
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
