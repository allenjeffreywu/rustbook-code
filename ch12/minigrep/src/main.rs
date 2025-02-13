// need to make cargo except multiple arguments
// something like: `$ cargo run -- searchstring example-filename.txt`
// to read commandline arguments, you might want to use crates.io in the future,
// but since we're learning this we'll want to do it ourselves
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// this file should only focus on running the program

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // notice that this prints the name of the binary being used. This is a C convention, we do not care about this.
    // dbg!(&args);

    // // saving argument values in variables
    // let config = parse_config(&args);

    let args: Vec<String> = env::args().collect();

    // unwrap or else is a special method that allows us to do some special error handling
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // reading a file
    // use if let here because we don't care about the returned value, (). better to only handle the error case with if let
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // // cargo run -- the poem.txt
    // println!("With text:\n{contents}");
}

// let's be explicitly clear with what the return is. Settling for a tuple is not good enough
struct Config {
    query: String,
    file_path: String,
}

// creating a constructor for config
impl Config {
    // more complicated constructor that returns a Result. We now require explicit error handling
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Box dyn error
// dyn here means dynamic
// box here is a trait object
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

// fn parse_config(args: &[String]) -> Config {
//     // the tradeoffs using clone
//     // has a large runtime cost, we are deep copying
//     // but it's better to have a working program that's a bit inefficient instead of early optimization
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }
