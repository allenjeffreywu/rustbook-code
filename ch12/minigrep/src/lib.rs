// this file should focus on the logic behind the program

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// defining search just enough for the test to compile
// lifetime variable 'a
// telling rust that the data returned by search function will ive as long as the data passed into the search function in the contents argument
// important because the data referenced by a slice needs to be valid for the reference to be valid. If the compiler assumes we are making string slices of query rather than contents, it will do its safety checking incorrectly.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Test Driven Development
// 1. Write a test that fails and run to make sure it fails for the reason you expect
// 2. Write or modify just enough code to make the new test pass
// 3. Refactor the code you just added or changed and make sure the tests continue to pass
// 4. Repeat from step 1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // the backslash after opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
