// Unrecoverable Error with panic!
// Rust has panic! macro. When this macro executes, the program will print a failure message
// unwind and clean up stack, then quit.
// Unwind the stack or Aborting in Response to a Panic

// let go to recoverable error with result
// enum Result<T, T> {
//     Ok(T),
//     Err(E),
// }
// the T and E is generic type.
// T represents the type of the value that will be returned in success case
// within the Ok variant.
// E represents the type of the error that will be returned in a failure case
// within the Err variant.
// example:
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;

fn main() {
    // panic!("crash and burn");

    // Using a panic! backtrace
    // let v = vec![1, 2, 3];
    // v[99];
    //

    // let f = File::open("example.txt");  // the return type of File::open is Result<T, E>
    // we need to add the code to take different actions depending on the value
    // File::open returns using match expression
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file: {:?}", error),
    // };

    // note that like the Option enum, the Result enum and its variants have been brought
    // into scope by the prelude, so we don't need to specify Result:: before the Ok and Err
    // variants in the match arm

    // matching different errors
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("example.txt") {  // if file not found, create file
    //             Ok(fc) => fc,
    //             Err(ec) => panic!("problem creating the file {:?}", ec),
    //         },
    //         other_error => {
    //             panic!("problem opening the file {:?}", other_error)
    //         }
    //     },
    // };

    // more concise way
    // these codes bellow doesn't contain any match expressions
    // use unwrap_or_else with closure
    let f = File::open("example.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("example.txt").unwrap_or_else(|error| {
                panic!("problem creating file {:?}", error);
            })
        } else {
            panic!("problem opening file {:?}", error);
        }
    });

    // shortcuts for Panic on error: unwrap and expect
    // Using match works well enough, but it can be a bit verbose and doesn't always
    // communicate intent well. If the Result value is Ok variant, unwrap will return
    // the value inside the Ok. If the Result value is Err variant, unwrap will call panic!
    //
    // let f = File::open("example.txt").unwrap();
    //
    // Another method, expect which is similar to unwrap, lets us also choose the panic! error
    // message. Using expect instead unwrap providing good error messages can convey your intent
    // and make tracking down the source of a panic easier.
    //
    // let f = File::open("example.txt").expect("Failed to open example.txt");

    // Propagating Errors
    // When writing a function whose implementation calls something that might fail, instead of
    // handling error within this function, you can return the error to the calling code so that it
    // can decide what to do.


    println!("{:?}", f);
}

// fn read_user_from_file() -> Result<String, io::Error> {
//     // let f = File::open("hello.txt");

//     let mut f = match File::open("hello.txt") {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// this function can be writen with a shorter way.
fn read_user_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // the ? place after Result value is defined to work in almost the same way as
    // the match expressions we defined to handle the Result.
    // more conciser
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // the main different between match and ? operator: error values that have ?
    // operator call on them go through the from function, defined in the From trait
    // which is used to convert errors from one type into another

    // Reading file into string is a fairly common operation, so Rust provides the
    // convenient fs::read_to_string function opens the file, creates a new string,
    // read the content of file into string and return it. Of course, fs::read_to_string
    // doesn't give the opportunity to explain all the error handling.
    // fs::read_to_string("hello.txt")

    //

}

// The ? operator can be used in Function that return Result
// fn main() {
//     let f = File::open("hello.txt")?;  // error when compile
//     // the ? operator is only allowed in a function that returns Result or Option or another
//     // type that implements std::ops::Try trait.
// }

// When using ? in a function that doesn't return one of these type, and you want to use ? when
// call other function that return Result<T, E>, you have 2 choices
// 1. change the return type of your function to be Result<T, E>
// 2. use match or one of the Result<T, E> methods to handle the Result<T, E>
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }
// The Box<dyn Error> type is called a trait object. Box<dyn Error> mean "any kind of error"
