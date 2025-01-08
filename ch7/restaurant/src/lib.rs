// this file was created with `cargo new restaurant --lib`

// module tree should be defined in src/lib.rs
// binary crate can access the public items within the library if this convention is followed (and use the starting paths with the nameof the package)
// binary crate becomes a user of the library crate

// paths for referring to an item in the module tree
// Can take two forms:
// 1. Absolute path = full path starting from a crate root, if from an external crate, it begins with the crate name, and if code from the current crate, it starts with the literal `crate`
// 2. relative path starts from the current module that uses self, super, or an identifier in the current module

// both absolute and relative paths are followed by one or more identifiers separated by double colons `::`

// we can access values in front_of_house from within eat_at_restaurant even tho front_of_house is private because eat_at_restaurant is a sibling
mod front_of_house {
    // hosting is nested inside of front_of_house.  is also a child of front_of_house
    // making the module public doesn't make its contents public
    pub mod hosting {
        // also need to make add_to_waitlist public
        pub fn add_to_waitlist() {
            // body of module goes here
        }

        // seat_at_table is a sibling of add_to_waitlist
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // pref using absolute paths because it's more likely we'll want to move code definitions and item calls independently of each other
    // doesn't compile because all modules are preivate by default - we must expose with pub
    // parent cant see private values of child, but child can see private values of parent
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Starting relative paths with super
// super of course refers to the parent
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // refers to the deliver order outside of the scope here
        super::deliver_order();
    }

    fn cook_order() {}

    // making structs and enums public
    // if we use pub before a struct defintion we make the struct public, but the struct's fields will still be private. you must specify which fields in the struct will be public

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // if we make an enum public, all of its variants are then public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
