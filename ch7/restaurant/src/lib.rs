// this file was created with `cargo new restaurant --lib`

mod front_of_house {
    // hosting is nested inside of front_of_house.  is also a child of front_of_house
    mod hosting {
        fn add_to_waitlist() {
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
