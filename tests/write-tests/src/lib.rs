// checking for panics with should_panic
// in addition to checking that our code returns the correct values we expect, it's also important
// to check that our code handles error conditions as we expect.
// should_panic attribute makes a test pass if the code inside the function panics; the test will
// fail if the code inside the function does not panic.
// we place #[should_panic] attribute after the #[test] attribute and before the test function it
// applies to
//
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {  // associated function
        if value < 1 {
            panic!("guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic!("guess value must be less than or equal to 100, got {}", value);
        }
        Guess{ value }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // using Result<T, E> in tests
    // so far, we've written tests that panic when they fail, we can also write tests that use
    // Result<T, E>.
    // in the body of the function, rather than calling the assert! macro, we return Ok(()) when
    // the test passes and an Err with a String inside when the test fails
    // we can't use the #[should_panic] annotation on tests that use Result<T, E>. instead we
    // should return and Err value directly when the test should fail.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn another() {
        panic!("make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle {width: 5, height: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle {width: 5, height: 1};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn its_adds_two(){
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "greeting did not contain name, values was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "guess value must be less that or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// the anatomy of a test function.
// at simplest, a test in Rust is a function that's annotated with the  test attribute. attributes
// are metadata about pieces of Rust code; one example is the derive attribute we used with structs
// to change a function into a test function, add #[test] on the line before of fn. when you run
// cargo test command, Rust build a test runner binary that runs the function annotated with the
// test attribute and reports on whether each test function passes or fails
//
// test fail when something in the test function panics. each test is run in a new thread, and when
// the main thread sees that a test thread has died, the test is marked as failed
//
// checking results with assert! macro
// the assert! macro, provided by the standard library, is useful when you want to ensure that some
// condition in a test evaluates to true. we give the assert! macro an argument that evaluates to a
// Boolean.
// if the value is true, assert! does nothing and the test passes. if the value is false, the
// assert! macro calls the panic! macro.
//
// testing equality with the assert_eq! and assert_ne! macros
// a common way to test functionality is to compare the result of the code  under test to the
// value you expect code to return to make sure they're equal. you could do this with assert! macro
// and passing it an expression using == operator. similarly way, we could use assert_eq! and
// assert_ne!
//
// under the surface, assert_eq! and assert_ne! macros use the operator == and != respectively.
//
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// testing aquality with the assert_eq! and assert_ne! macros

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// adding custom failure messages
// you can also add a custom message to be printed with the failure message as optional arguments
// to the assert!, assert_eq!, assert_ne!. any arguments specified after the one required argument
// to assert! or the two required arguments to assert_eq! and assert_ne! are passed along to the
// forformat! macro, so you can pass a format string that contains {} placeholders and values to go
// in those placeholders
pub fn greeting(name: &str) -> String {
    format!("hello {}!", name)
}
