// test organization
//
// Rust community thinks about tests in terms of two main categories: unit tests and integration
// tests. unit tests are small and more focused, testing one module in isolation at a time, and can
// test private interfaces.
// integration tests are entirely external to your library and use your code in the same way any
// other external code would, using only the public interface and potentially exercising multiple
// modules per test.
//
// unit tests
// the purpose of unit tests is to test each unit of code in isolation from the rest of the code to
// quickly pinpoint where code is and isn't working as expected. you'll put unit tests in the src
// directory in each file with the code that they're testing. the convention is to create a module
// named tests in each file to contain the test functions and to annotate the module with cfg(test)
//
// the tests module and #[cfg(test)]
// the #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only
// when you run cargo test, not when run cargo build. this saves compile time when you only want to
// build the library and saves space in the resulting compiled artifact because the tests are not
// include.
// you'll see that because integration test go in a different directory, they don't need the
// #[cfg(test)] annotation. however, because unit tests go in the same files as the code, you'll
// use #[cfg(test)] to specify that they shouldn't be included in the compiled result.
// by using cfg attribute, Cargo compiles our test code only if we actively run the tests with
// cargo test. this includes any helper functions that might be within this module, in addition to
// the functions annotated with #[test].
//
// testing private functions
// there's debate within the testing community about whether or not private functions should be
// tested directly, and other languages make it difficult or impossible to test private functions.
//
pub fn add_two(a: i32) -> i32 {
    a + 2
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// note that internal_adder function is not marked as pub, but because tests just Rust code and the
// tests module is just another module, you can bring internal_adder into a test's scope and call
// it. if you don't think private functions should be tested, there's nothing in Rust that will
// compel you to do so.
//
//
// integration tests
// in Rust, integration tests are entirely external to library. they use your library in the same
// way any other code could, which means they can only call functions that are part of your
// library's public API.
//
// the tests directory
// creat tests/integration_tests.rs
