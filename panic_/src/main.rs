// Unrecoverable Error with panic!
// Rust has panic! macro. When this macro executes, the program will print a failure message
// unwind and clean up stack, then quit.
// Unwind the stack or Aborting in Response to a Panic

fn main() {
    panic!("crash and burn");

    // Using a panic! backtrace
    let v = vec![1, 2, 3];
    v[99];
}
