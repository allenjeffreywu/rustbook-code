// need to make cargo except multiple arguments
// something like: `$ cargo run -- searchstring example-filename.txt`
// to read commandline arguments, you might want to use crates.io in the future,
// but since we're learning this we'll want to do it ourselves
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // notice that this prints the name of the binary being used. This is a C convention, we do not care about this.
    dbg!(&args);

    // saving argument values in variables
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // reading a file

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
