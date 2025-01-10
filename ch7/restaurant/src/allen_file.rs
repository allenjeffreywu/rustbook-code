// this works because we have a reference to this file in lib.rs
// mod is not an include operation that is in other programming language
// there are more variations of this
// we can make allen_file.rs be a direcory and have working.rs with the on_a_computer function as the only member in it and it would be valid
pub mod working {
    pub fn on_a_computer() {}
}
