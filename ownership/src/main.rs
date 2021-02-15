// each value in Rust has a variable that's called its owner
// there can only be one owner at a time
// when the owner goes out of scope, the value will be dropped

/*
fn main() {
    let mut s = String::from("hello"); // s is valid from this point forward
    s.push_str(", world"); // do stuff with s
    println!("{}", s);
} // s will go out of its scope here, and the value of s will be dropped
*/

fn main() {
    /*
    let s1 = String::from("hello");
    let s2  = s1;  // s1's value will be moved to s2. s1 no longer exists here
    */

    /*
    let x = 5;
    let y = x;  // because the value of x is scalar type, so it's stored on stack => x's value is cloned to y
    */

    /*
    // we can borrow by use the ampersand symbol
    let s = String::from("hello");
    let s1 = &s; // s1 is borrowing the value of s, not owns it.
                 // we can have multiple (immutable) references
    let s2 = &s; // it's ok
                 // but we can have only one mutable references at a time
                 //
    */
    let s = String::from("hello world");
    let slice = first_word(&s);
    println!("{}", slice);
    let my_string = String::from("hello phuoc");
    let word = first_word(&my_string);
    println!("{}", word);

    // type of string literals are &str
    //
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // return a reference to s
}  // but s will be dropped here
*/

/*
 fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
