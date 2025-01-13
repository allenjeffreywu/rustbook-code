// Collections are stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs
// 3 useful ones
// 1. vector - like arraylist
// 2. String - important capital S
// 3. Hashmap - map

fn main() {
    println!("Hello, world!");

    // Vectors
    let v: Vec<i32> = Vec::new();
    // you can have rust infer the type that you have in the vector with this macro
    let v = vec![1, 2, 3];

    // updating a vector

    let mut v = Vec::new();
    // add to the end of a vector with push
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // read elements of vectors with get or indexing
    let v = vec![1, 2, 3, 4, 5];

    // indexing, and using as refernce to not take ownership
    // this can crash if we go out of bounds
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    // with get we receive an option, so we must use match
    // use this option if you want a nicer user experience.. lol
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // the following does not compile:
    /*
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
     */

    // why? because when we add to a vector we might actually move this data somewhere else on the heap. So the reference becomes invalid.

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // using an enume to store multiple types
    // we can store enums in vectors!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // like with any other struct, a vector is freed when it goes out of scope

    // String (Storing UTF-8 Encoded Text with Strings)
    // What is a String? Rust only has one string type in the core language, which is the string slice str.
    // The String type, which is provided by Rust's standard library rather than coded into the core language, is different. Though both use UTF-8 strings

    // Creating a new String
    // String is actually implemented as a wrapper around a vector of bytes with some extra guarantees.
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // we can also use String::from to create a String from a string literal
    let s = String::from("initial contents");

    // you can use from and to_string interchangeably, they do the same thing

    // Updating a string

    // push_str - uses a string slice to not take ownership
    let mut s = String::from("foo");
    s.push_str("bar");

    // push - adds a single character to the vec
    let mut s = String::from("lo");
    s.push('l');

    // + operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // + operator uses the add method, which uses self for s1
    // s2 here is string coercion. We are taking &String and forcing it into &str

    // we don't like using too many + operators
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // so we can use format! instead
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // uses references so that this call doesn't take ownership of any of its parameters

    // Indexing into strings
    // not possible in rust
    /*
    let s1 = String::from("hello");
    let h = s1[0];
     */
    // I love safety
    // Internal representation is UTF-8 so some characters are not the same size as others, so indexing makes no sense.

    // Bytes and Scalar Values and Grapheme Clusters (aka letters)
    // letters to make a word are not all the same in every language. Hindi being one of them

    // Slicing strings
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // still possible, but you have to know how it is all done in UTF-8

    // Methods for iterating over Strings

    // using .chars() - will give you each element as expected even if multibyte
    for c in "Зд".chars() {
        println!("{c}");
    }

    // using .bytes - will return each raw byte
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // there are also contains and replace methods as well in rust.

    // Hash Maps

    // Creating a new Hash Map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a hash map - get method
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get returns Option<&V>. If it doesn't exist it returns none
    // copied returns an Option<i32> instead of Option<&i32>
    // unwrap_or to set score to 0 if scores doesn't have an entry for the key. Clever function

    // can loop over all key-value pairs with a for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // will return in an arbitrary order

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // if a type has the Copy trait, then it will be COPIED into the hash map

    // Updating a hash map - you know how to do this. Decide to keep the old value or not

    // Overwriting a value
    // inserting two of the same key into a hashmap will overwrite the key
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // adding a key and value if a key isn't present using the entry api - tells you if the entry exists or not
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    // or_insert method on entry is defined to return a mutable reference to the value for the corresponding entry key if that key exists. If not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    // so if Blue already exists in the hashmap, it will return its value and NOT insert 50 in the value.

    // Updating a value based on the old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace method retunrs an iterator over subslices separated by whitespace
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // updates the value at that address!
    }

    println!("{map:?}");
    // the HashMap uses siphash as default hashing function
}
