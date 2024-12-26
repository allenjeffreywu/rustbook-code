fn main() {
    // ownership model, completely different from garbage collection and memory management you may be used to
    // Stack and Heap comparch review
    // Stack = stores values in the order it gets them (hence the name) LIFO. All data here has a fixed size
    // Heap = all data here has an unknown size. malloc for space, returns a pointer. You can store this pointer on the stack, but the actual data is on the heap
    // as usual, heap is slower than the stack

    // ownership rules
    // each value in rust has an owner
    // There can only be one owner at a time
    // when the owner goes out of scope, the value will be dropped

    // variable scope
    // this variable is in scope for the entire main function after it is declared here
    let s = "hello";

    // the String type
    // two types - string and String. String (capital S) is used to manage data allocated on the heap an store unknown text to us at compile time.

    // convert string to String with:
    let mut s = String::from("hello");
    // is mutable
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    // Memory and Allocation
    // drop is what is called to clean up a variable when it goes out of scope

    // variables and data interacting with Move
    let x = 5;
    let y = x;
    // we copy x value to y value and they both equal 5

    let s1 = String::from("hello");
    let s2 = s1;
    // this is different s1 and s2 are both pointers pointing to the same bit of memory.
    // if s1 or s2 go out of scope they will try to free the same bit of memory, and we lead to double free

    // so instead what happens is the MOVE operation
    // when s2 = s1 is called, s1 is MOVED to s2 and we can no longer reference it
    // println!("{s1}, world!"); // does not work
    // no more automatic deep copy

    // scope and assignment
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
    // when s is reassigned to ahoy we immediately drop the hello from the heap

    // variables and data interacting with clone
    // if we want to deeply copy string we do clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // stack only data: copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // since x and y are both integers, they are stack only so they will be automatically copied
    // rust also has a copy annotation that we can modify to have special behavior for stack only variables.
    // cannot be used when the type has implemented the drop trait (of course lol)

    // ownership and functions

    takes_ownership(s); // s's value moves into the function and is no longer valid after this line

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's ok to still use x afterward. (primitive)

    // return values and scope
    // returning values gives back ownership
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3 (AND THIS VALUE INVALIDATES s2)
} // s is now no longer in scope.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
