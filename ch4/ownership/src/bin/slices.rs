// `cargo run --bin slices`

fn main() {
    // slices let you reference a contigious sequence of elements in a collection rather than the whole collection
    // slice is a kind of reference, and therefore does not have ownership

    // we would rather not do substring and have the integer value for the end of a word get out of sync with the string
    // so we introduce slices
    let s = String::from("hello world");

    // ngl this is kinda what I expected for strings. it just makes sense
    let hello = &s[0..5];
    let world = &s[6..11];

    // we could also do
    let slice = &s[..2];
    // because it assumes 0 at the beginning
    // same for the end - it just assumes the end
    let slice = &s[3..];

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {word}");

    // String literals as slices
    let s = "Hello, world!";
    // s has type &str, and is a SLICE pointing to that specific point of binary. immutable. clever

    // string slices as parameters
    // we can improve the method header to fn first_word(s: &str) -> &str {
    // so that it can be used on &Str AND &str
    // this flexibility takes advantage of deref coercions

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // other slices
    // we can use them for other arrays just like python
    let a = [1, 2, 3, 4, 5];
    // also not inclusive of the last index..
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
