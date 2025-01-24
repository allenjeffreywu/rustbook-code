// run with cargo test.
// 3 sections now.
// unit test
// integration test
// documentation test

// run directly with cargo test --test integration_test
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

// you can add submodules in integration tests

//if project is a binary crate with only a main.rs, then we can't create integration tests. have a lib.rs file and use the main to call it. then we can test
