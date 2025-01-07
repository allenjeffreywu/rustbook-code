// CRATE

// crate = smallest amount of code the rust compiler considers at a time
// rust considers single files to be a crate
// crates come in two forms: binary crate or a library crate

// binary crates are programs you can compile to an executable that you can run. requires a main function

// library crate don't have a main function. They define functionality intended to be shared with multiple projects (like rand). This is what most people refer to when they describe crates

// crate root is a source file that the rust compiler starts from and makes up the root module of your crate

// PACKAGE

// package = bundle of one or more crates that provide a set of functionality
// packages contain a cargo.toml file that describes how to build crates
// cargo is a package that contains the binary crate for the CLI tool
// packages can contain as many binary crates as you would want, but most are only one library crate
// the src directory contains main.rs and Cargo.toml. The Cargo.toml file does not have any indicator that src/main.rs is the root. By convention it just knows

// for library packages, it knows that src/lib.rs is the crate root
// currently we only have main.rs. That means it only has a binary crate named my-project

// if we add lib.rs we will have two crates, a binary and a library - both named my-project.

// if we add a src/bin directory we can add multiple binary crates by creating files in there. Each file will be a separate binary crate

// Modules Cheat Sheet

// Start from the crate root: when compiling, the compiler first looks at the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate)

// Declaring Modules: in the crate root file, you can declare new modules
mod garden;
// the compiler will look for the module's code in:
// 1. Inline, within curly brackets that replace the semicolon following `mod garden`
// 2. In the file src/garden.rs
// 3. In the file src/garden/mod.rs

// Declaring submodules: In any file other than the crate root, you can declare submodules. (see src/garden.rs)

// Paths to code in modules: You can refer to code in a module from anywhere else in the same crate as long as the privacy rules allow
use crate::garden::vegetables::Asparagus;

// private vs. public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod (see garden.rs). To make items within a public module public as well, use pub before their declarations (see vegetables.rs)

// use keyword: creates shortcuts to items to reduce repetition of long paths

fn main() {
    // shortened thanks to the use keyword
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
