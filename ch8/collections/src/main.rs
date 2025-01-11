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
}
