// controlling how tests are run
//
// just as cargo run compiles code and then runs the resulting binary, cargo test compiles code in
// test mode and runs the resulting test binary. you can specify command line options to change the
// default behavior of cargo test.
//
//
// running tests in parallel or consecutively
//
// when run multiple tests, be default they run in parallel using threads. this means the tests
// will finish faster so you can get feedback quicker on whether or.
// tests running at the same time => make sure your tests don't depend on each other or on any
// shared state, including shared environment, such as current working directory or environment
// variables
// if you don't want to run the tests in parallel or if you want more fine-grained control over the
// the number of threads used, you can send the --test-threads flag and the number of threads you
// want to use. ex: cargo test -- --test-threads=1
//
// showing function output
// by default, if the test passes, Rust's test library captures anything printed to standard output
// ex: if we call println! in a test and the test passes, we won't see the println! output in the
// terminal.
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
// note that nowwhere in this output do we see I got the value 4, which is that is printed when the
// test that passes runs. that output has been captured.
// if we want to see printed values for passing tests as well, we can send --show-output flag to
// the command cargo test. ex: cargo test -- --show-output

// runing a subset of tests by name
// somtimes, running a full test suite can take a long time. if you're working on code in a
// particular area, you might want to run only the tests pertaining to that code. ex:
fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // ignoring some tests unless specically requested
    // sometimes a few specific tests can be very time-consuming to execute, so you might want to
    // exclude them during mosts run of cargo test. rather than listing as arguments all tests you
    // do want to run, you can instead annotate the time-consuming test the ignore attribute to
    // exclude theme.
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to  run
    }
    // if we want to run only the ignore tests, we can use: cargo test -- --ignored
}

// running single tests
// we can pass the name of any test function to cargo test command.
// ex: cargo test one_hundred
//
// filtering to run multiple tests
// we can specify part of a test name, and any test whose name matches that value will be run
// ex: cargo test add
//
// by controlling which test run, you can make sure your cargo test result will be fast.

// & vs ref
// & denotes that pattern expects a reference to an object. hence & is a part of said pattern: &Foo
// matches different objects than Foo does
// ref indicates that we want a reference to an unpacked value. it's not matched against
// Foo(ref foo) matches the same object as Foo(foo)
