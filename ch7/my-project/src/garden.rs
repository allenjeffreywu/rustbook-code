// this declares a submodules
pub mod vegetables;
// The compiler will look for the submodule's code within the directory named for the parent module in these places:
// 1. Inline directly following `mod vegetables`, within curly brackets instead of the semicolon
// 2. In the file src/garden/vegetables.rs
// 3. In the file src/garden/vegetables/mod.rs
