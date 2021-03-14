use generics_traits::{self, Tweet, Summary, notify, other_notify, longest};
use std::cmp::PartialOrd;
use std::fmt::Display;

// implement generics
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


// we can fix the problem of largest function in generic with traits bounds syntax
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list {
        if item > largest {  // error here
            largest = item;
        }
    }

    largest

    // if we don't want to restrict largest function to the types that implement the Copy trait, we
    // could specify that T has the trait bound Clone instead of Copy. then we could clone each
    // value in the slice when we want largest function to have ownership. using clone function that means we're potentially
    // making more heap allocations in the case of types that own heap data like String -> it can
    // be slow if we've working with a large of data
}


// using trait bounds to conditionally implement methods
// by using a trait bound with an impl block that uses generic types parameters, we can implement
// methods conditionally for types that implement the specific traits.

struct Pair<T> {
    x: T,
    y: T,
}

// the Pair<T> always implements the new method
// but the cmp_display method only be implemented when its inner type T implements the PartialOrd
// trait for comparison and the Display trait for printing
//

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {  // concrete for -> Pair<T>
        Self {x, y}
    }
}


impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}

// we can also conditionally implement a trait for any type that implements another trait.
// implementations for a trait on any type that satisfies the trait bounds are called blanket
// implementations and are extensively used the Rust standard library
// ex: the standard library implements the ToString trait on type that implements the Display trait
// impl<T: Display> ToString for T {
// // snip
// }
// because the standard library has this blanket implementation, we can call the to_string method
// defined by the ToString trait on any type that implements the Display trait. ex:
// we can turn integers into their corresponding String values like this because integers implement
// Display trait.
// let s = 3.to_string();
//
// blanket implementation appear in the documentation for the trait in the implementors section
// traits and trait bounds let us write code that uses generic type parameters to reduce
// duplication but also specify to the compiler that we want the generic type to have particular
// behavior. the compiler can then use the trait bound information to check that all the concrete
// types used with our code provide the correct behavior.
// in dynamically typed languages, we would get an error at runtime if we call method on a type
// which didn't define the method. Rust moves these errors to compile time.
// additionally, we don't have to write code that checks for the behavior at runtime because we've
// already checked at compile time.
//
// another kind of generic that we've already been using is called lifetimes. rather than ensuring
// that a type has the behavior we want, lifetimes ensure that references are valid as long as we
// need them to be


// every references in Rust has a lifetime, which is the scope for which that reference is valid.
// most of times, lifetimes are implicit and inferred, just like most of the time, type are
// inferred. we must annotate types when multiple types are possible.
// similarly, we must annotate lifetimes when the lifetimes of references could be related in a few
// different ways.
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the
// actual references used at runtime will defintely be valid
//
// lifetime annotations in struct definiton
//
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime elision
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules,
// These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider,
// and if your code fits these cases, you don’t need to write the lifetimes explicitly.
// the elision rules do not provide full inference
//
// The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations
// the first rule applies to input lifetimes, and the second and third rules apply to output lifetimes
// f the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes,
// the compiler will stop with an error
// These rules apply to fn definitions as well as impl blocks.
// The first rule is that each parameter that is a reference gets its own lifetime parameter
// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self
// because this is a method, the lifetime of self is assigned to all output lifetime parameters
// This third rule makes methods much nicer to read and write because fewer symbols are necessary
// ex:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
// this signature will be applied the first rules, but second and third rules are not applied
//
// lifetime annotations in method definitions
//
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {  // apply the third rule here
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {}", announcement);
        self.part
    }
}
// because of third rule, the return value of method level and announce_and_return_part will have
// the lifetime of &self
//
// generic types parameters, trait bound, and lifetimes together
// Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// or use this signature
// fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
//     println!("announcement: {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn main() {
    let p1 = Point{x: 5.0, y: 10.4 };
    let p2 = Point{x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let tweet = Tweet{
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probaly already know, people"),
    reply: false,
    retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    other_notify(&tweet);

    let numbers = vec![34, 50, 25, 100, 65];
    println!("the largest number is {}", largest(&numbers));

    let lg = Pair::new(3, 5);
    lg.cmp_display();

    let string1 = String::from("abcd");
    let string2 = "xya";
    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportantExcerpt{ part: first_sentence };
    println!("level i: {:?}, i: {:?}", i.level(), i.announce_and_return_part("hello"));


    // static lifetimes
    // 'static that mean this reference can live for entire duration of the program. all string literal
    // have 'static lifetime
    let s: &'static str = "I have a static lifetime";
    // You might see suggestions to use the 'static lifetime in error messages
    //  But before specifying 'static as the lifetime for a reference, think about
    //  whether the reference you have actually lives the entire lifetime of your program or not.
    //  You might consider whether you want it to live that long, even if it could
    //  Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes
    //   In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
    println!("longest with ann: {}",longest_with_an_announcement(string1.as_str(), string2, String::from("hello")));
}
