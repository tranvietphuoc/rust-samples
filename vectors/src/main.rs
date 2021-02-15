use std::collections::*;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..10{
        v.push(i);
    }
    println!("{:?}", v);

    let v1 = vec![1, 2, 3, 4];
    let third = &v1[2];  // index must be usize
    println!("the third element is {}", third);

    match v1.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element"),
    }

    let v2 = &v[100];  // panic here
    let v3 = v1.get(100);  // None here

    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    v.push(6);
    println!("the first element is {}", first);  //error - can't borrow v4 as mutable because it's also borrowed as immutable

    // iterating over the value in a vector
    let v5 = vec![100, 200, 300];
    for i in &v5 {  //immutable reference
        println!("{}", i);
    }

    let mut v6 = vec![100, 200, 300, 400];
    for i in &mut v6 {  // mutable reference to change elements of v6
        *i += 1;
    }

    // using an enum to store multiple type
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(4.0)
    ];
    // Rust need to know what types will be in the vector at compile time
    // so it knows exactly how much memory on the heap will be need to store each element.
    // A secondary advantage is that we can be explicit about what types are allowed in vector
    // using enum and match expression make Rust will ensure at compile time
    // if you don't know the exhautive set of types the program will get at
    // runtime to store in a vector, the enum technique won't work

}
