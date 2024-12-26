// what if we want a function to use a variable without taking ownership? we use references

// you can run this file with `cargo run --bin references`
fn main() {
    let s1 = String::from("hello");

    // this is a reference
    // unlike pointers, a reference is guaranteed to point to a  valid value of a particular type for the life of that reference
    // we call the action of creating a reference (with &) as borrowing. if a person owns something, you can borrow it from them
    // YOU CANNOT MUTATE REFERENCES NORMALLY.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // opposite of reference is dereferencing. Use the normal dereference operator * to do so.

    // to mutate references you must do the following

    // s here must be mut
    // there is one restriction to this
    // if you have a mutable reference to a value you can have NO other references to that value (scoping still applies. If it falls out then you can)
    let mut s = String::from("hello");

    // which allows us to create a mutable reference
    change(&mut s);

    // we solve data races with this design! (like race conditions, where programs fight for the same reference)
    // 1. Two or more pointers access the same data at the same time
    // 2. at least one of the pointers is being used to write to the data
    // 3. there's no mechanism being used to synchronize access to the data

    // we cannot have mutable references while we have an immmutable reference to the same value.
    // multiple immutable referencs are allowed because no one who is just reading the dat has the ability to affect anyone else's reading of the data.

    let s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    // this is legal tho
    // reference scope starts from where it is introduced and continues through the last time that reference is USED. <- keyword USED!!
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point - dropped

    let r3 = &mut s; // no problem
    println!("{r3}");

    // dangling references
    let reference_to_nothing = dangle(); // goto dangle
}

// also have to specifically state that the parameter s here is for REFERENCES.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s // we return a reference to the String, s
    // solution is to return the string DIRECTLY
    s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
