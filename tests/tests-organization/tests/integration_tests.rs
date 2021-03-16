extern crate tests_organization;

mod common;

// create tests/integration_tests.rs in the same level of src/ folder.
// we've added tests_organization at the top of the code. which we didn't need in the unit tests
// the reason is that each file in the tests directory is a separate crate, so we need to bring our
// library into each test crate's scope

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests_organization::add_two(2));
}

// the three sections of output include the unit tests, the integration tests, and the doc tests.
// the first: the same as we've been seeing: one line for each unit test (one named internal).
// the integration tests sections start with the line Running
// /target/debug/deps/integration_tests-....
// next, there is a line for each test function in that integration test and summary line for the
// results of the integration test just before the Doc-tests integration_tests section starts
//
// similarly to how adding more unit test functions adds more result lines to the unit tests
// section, adding more test functions to the integration test file adds more result lines to this
// integration test file's section. each integration test file has its own section, so if we add
// more files in the tests directory, there will be more integration test sections.
// we can still run a particular integration test function by specifying the test function's names
// as an argument to cargo test. ex: cargo test -- --test integration_tests. this command runs only
// the tests in the tests/integration_tests.rs file
//
// submodules in integration tests
//
// as you add more integration tests, you might want to make more than one file in the tests
// directory to help organize them; ex: create tests/common.rs
//
// note that mod common; declaration is the same  as the module declaration we demonstrated.
// then in the test function, we can call the common::setup() function.
//
// integration tests binary crates
//
// if our project is a binary crate that only contains a src/main.rs file and doesn't have a
// src/lib.rs file, we can't create integration tests in the tests directory and bring functions
// defined in the src/main.rs file into scope with a use statement. only library crates expose
// functions that other crates can use; binary crates are meant to be run on their own
//
// this is one of the reason Rust project that provide a binary have a straightforward src/main.rs
// file that calls logic that lives in the src/lib.rs file. using that structure, integration tests
// can test the library crate with use to make the important functionality available. if the
// important functionality works, the small amount of code in the src/main.rs file will work as
// well, and that small amount of code doesn't need to be tested
