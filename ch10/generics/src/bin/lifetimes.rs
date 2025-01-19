// you can run this file with `cargo run --bin lifetimes`

// validating references with lifetimes

// This won't compile. Why? Because we don't know if x or y will be returned
// they are both references and may be drastically different.
// We also don't know the lifetimes of the references passed in so this could be dangerous
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// adding generic lifetime parameters that define the relationship between the references help the borrow checker understand.

// lifetime annotation syntax
// names of lifetime parameters must start with an apostrophe (')
// THIS DOES NOT CHANGE HOW LONG ANY OF THE REFERENCES LIVE
/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

// lifetime annotations in function signatures

// The returned reference will be valid as long as both the parameters are valid
// do not put annotations in the function body
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// We don't need lifetimes for everything
// The reason why this compiles without lifetime annotations is historical
// Rust team added this pattern to the compiler because programmers kept doing it lmao
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // preventing dangling references with lifetimes
    /*
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
     */
    // the above is invalid
    // we know this because x falls out of scope
    // Rust knows this because it has a borrow checker
    // remove the curly braces and the code will compile

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
        // string 2 ends here
    }

    // the following does not compile for obvious reasons:
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
     */

    // thinking in terms of lifetimes
    /*
    fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
    }
     */
    // the above fails to compile because we don't give any of the parameters lifetimes and return them.

    // Lifetime annotations in struct definitions
    // we can define structs to hold references, but we NEED to add a lifetime annotation on every reference
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i: {0}", i.part);

    // Lifetime Elision
    // look at first_word
    // Compiler uses 3 rules to figure out the lifetimes of the references when there aren't explicit annotations
    // 1 input life times, 2nd and 3rd are for output lifetimes
    // 1st: compiler assigns a lifetime parameter to each parameter that's a reference
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    // 2nd: if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // fn foo<'a>(x: &'a i32) -> &'a i32
    // 3rd if there are multiple input lifetime parameters, but one of them is &self or &mut self because it is a method, the lifetime of self is assigned to all output lifetime parameters

    // lifetime annotations in method definitions
    /*
    impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
            3
        }
    }
     */
    // after impl and everything is great, but no lifetime reference to self needed b/c of 1st rule

    // where 3rd applies
    /*
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }
     */
    // two input lifetimes, but one of them is self, so the output return type gets the same lifetime as self

    // static lifetime
    // SPECIAL LIFETIME CALLED 'STATIC
    // IT CAN LIVE FOR THE ENTIRE DURATION OF THE PROGRAM
    let s: &'static str = "I have a static lifetime.";

    // putting it all together
    /*
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
     */
    // insane

    println!("Hello Allen");
}
