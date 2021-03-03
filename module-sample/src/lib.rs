// use external package
// use std::{io, cmp::Ordering};  // use nested paths
use std::collections::*;  // if we want to bring all public item. Use glob operator
// the glob operator is often used when testing to bring everything under test into the tests
// module
use std::fmt::Result;
// use std::io::Result as IoResult;  // use with as keyword
// use std::io;
// use std::io::Write;
use std::io::{self, Write};  // use self to use two paths into one use statement


// when we bring a name into scope with the use keyword, the name available in the new scope is
// private
// To enable the code that calls our code to refer to that name as if it had been
// defined in that code's scope, we can combine pub with use. This technique call re-exporting
//
//
// // absolute path
// pub use crate::front_of_house::*;

// relative path
// front_of_house::hosting::add_to_waitlist();

// if you compile this, you'll get error, because hosting is private
// so, you need to add pub keyword at start of hosting definition and add_to_waitlist function
// mod front_of_house;  // seperate mod into many files, declare mod front_of_house end with ;

// re-structed modules follow directory. inside the directory, it must have mod.rs file
// inside mod.rs file must have mod statement
// inside the current file, which import the directory as module must have mod statement too.
mod front_of_house;
pub use front_of_house::*;  // then use mod hosting of front_of_house - re-exporting

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    //
}

// // starting relative paths with super
fn serve_order() {}

mod back_of_house_ver1 {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // use super:: to create a realtive path from parent scope
    }

    pub fn cook_order() {}
}
//
// making struct and enum public

#[warn(dead_code)]
pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,  // private field
    }

    // to use enum in module we only need the pub keyword before enum keyword.
    //
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
