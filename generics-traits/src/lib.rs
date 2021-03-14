use std::fmt::Display;
use std::fmt::Debug;


// traits: defining shared behavior
// defining trait:
// a type behavior consists of the methods we call on that type.
// different types share the same behavior if we can call the same methods on all of those types
// trait definitions are a way to group method signatures together to define a set of behavior
// necessary to accomplish some purpose
pub trait Summary {
    fn summarize_author(&self) -> String;  // this is method signature, it describes the behavior of the types that implement this trait

    // a trait can have multiple methods in its body: the method signatures are listed one per line
    // and each line ends in a semicolon.
    // default implementations can call other methods in the same trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())  // default implementation
    }  // method signatures describe the behaviors of the types that implement this trait
}

// to use default implementation of Summary trait instead of defining a custom implementation,
// we specify an empty impl block with impl Summary for NewArticle{}

// each types that implementing this trait must provide its own custom behavior for the body of the
// method. let's see two structs bellow.
// implement a trait on a type
// now we implement the behavior of Summary trait on type in our media aggregator
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{})", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// implement a trait on a types is similar to implementing regular methods. the different is that
// after impl, we put the trait name that we want to implement, the use for keyword.
//
//
// traits are similar to a feature often called interfaces in other languages.
// Note that because we defined the Summary trait and these structs in the same lib.rs, they are in
// the same scope.
// this lib.rs for a crate we've called aggregator and someone else wants to use our crate's
// functionality to implement Summary trait on a struct defining with their library's scope.
// by specifying use aggregator::Summary; the Summary trait also need to be a public trait for
// another crate to implement it.
//
// one restriction to note with trait implementations is that we can implement trait on a type only
// if either the trait or the type is local to our crate. ex: we can implement standard library
// traits like Display on a custom type like Tweet as a part of our aggregator crate functionality,
// because the type Tweet is local to our aggregator crate. we can also implement Summary on Vec<T>
// in our aggregator crate.
// but we can't implement external traits on external types. ex: we can't implement Display on
// Vec<T> within our aggregator crate, because Display and Vec<T> are defined in standard library
// and aren't local to our aggregator crate. this restriction is part of a property called
// coherence, and more specially the orphan rule. so named because parent type is not present


// traits as parameters
//
// we can explore how to use traits to define functions that accept many different types
// instead of a concrete type for the item parameter, we can specify the impl keyword and trait
// name. this parameter accepts any type that implements the specific trait. in the body of notify,
// we can call any methods on item that come from the Summary trait. we can call, and pass any
// instance of NewArticle or Tweet. on other types such as String, i32, i64, f64,... Rust won't
// compile because those types don't implement Summary
pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

// trait bound syntax
//
// the impl Trait syntax works for straightforward cases but it actually syntax sugar for a longer
// form, which is called trait bound, look like this:
// this form is equivalent to the previous form of notify. we can place trait bounds with the
// declaration of the generic type parameter after a colon and inside angle brackets
pub fn other_notify<T: Summary>(item: &T) {
    println!("breaking news! {}", item.summarize());
}
// the trait bound syntax can express more complexity in other case
pub fn other_notify_long(item1: &impl Summary, item2: &impl Summary) {
    //
}
pub fn other_notify_long_better<T: Summary>(item1: &T, item2: &T) {
    //
}

// specifying multiple trait bounds with the + syntax
// we can also specify more that one trait bound. say we wanted notify to use  display formatting
// on item as well as the summarize method: we specify in the notify definition that item must
// impleimplement both Display and Summary trait with + syntax
pub fn multiple_notify(item: &(impl Summary + Display)) {}
// + with trait bound
pub fn multiple_notify_better<T: Summary + Display>(item: &T){}

// clearer trait bounds with where clauses
// using too many trait bounds has its downsides. each generic has its own trait bounds, so
// functions with multiple generic type parameters can contain lots of trait bound information
// between the function's name and its parameter list, making the function signature hard to read
// Rust has alternate syntax for specifying trait bounds inside a where clause after function
// signature
// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//    //
// }
// with where clause:
// this function's signature is less cluttered, the function name, parameter list, and return type
// are close together, similar to a function without lots of trait bounds
//
// pub fn some_function_where<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
// {
//     //
// }
//
//
// return types that implement traits
// we can also use the impl trait syntax in the return position to return a value of some type that
// implements a trait, as shown here:

fn returns_summarizale() -> impl Summary {  // the returns some type that implements the Summary trait without naming the concrete type.
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"),
        reply: false,
        retweet: false,
    }
}

// the ability to return a type that is only specified by the trait it implements is especially
// useful in the context of closures and iterators.
// the impl Trait syntax let us concisely specify that a function returns some type that implements
// the Iterator trait without needing to write out a very long type.
//
//
//
// Self is the type of current object. it appears in trait or impl. it's syntastic sugar for the
// recieving type of method, ex:
// impl Clone for MyType {
// I can use either the concrete type (known here)
//  fn clone(&self) -> MyType;

// Or I can use Self again, it's shorter after all!
//  fn clone(&self) -> Self;
// }
// self => self: Self
// &self => self: &Self
// &mut self => self: &mut Self
//
// the borrow checker
// Rust compiler has a borrow checker, that compares scopes to determine whether all borrows are
// valid.
/*
 * {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
*/
// here we've annotated the lifetime of f with 'a and the lifetime of x with 'b.
// at compile time, Rust compares the size of two lifetimes and sees that r has a lifetime of 'a
// but that it refers to memory with a lifetime of 'b. the program is rejected because 'b is
// shorter that 'a.
/*
 * {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
*/
// here, x has the lifetime 'b, which in case is larger than 'a. this mean r can refer x because
// Rust knows that the reference in r will always be valid while x is valid.
//
// lifetimes annotation syntax
// lifetime annotation don't change how long any of the reference live. just as functions can
// accept any type when the signature specifies as a generic type parameter, function can accept
// references with any lifetime by specifying a generic lifetime parameter.
// lifetime annotation describe the relationships of the lifetimes of multiple references to each
// other without affecting the lifetimes
// ex: lifetime annotations in function signatures

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// with the function longest above, if we don't add 'a annotation to the function, Rust compiler
// will not compile
//
//
// thinking the term of lifetimes
// the way in which we need to specify lifetime parameters depends on what our function is doing.
//
//
// const VERSION: i32 = 1;
// static MINOR_VERSION: i32 = 0;
// const live for the entire lifetime of program, and it value will be inlined into any place the
// code refers to it
// static also lives for the entire lifetime of program, but it isn't inlined in code.
//
