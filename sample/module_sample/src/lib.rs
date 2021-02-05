// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// use external package
// use std::{io, cmp::Ordering};  // nested paths
// use std::collections::*;
// // absolute path
// pub use crate::front_of_house::*;

// relative path
// front_of_house::hosting::add_to_waitlist();

// if you compile this, you'll get error, because hosting is private
// so, you need to add pub keyword at start of hosting definition and add_to_waitlist function
mod front_of_house;  // seperate mod into many files, declare mod front_of_house end with ;
pub use crate::front_of_house::hosting;  // then use mod hosting of front_of_house

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    //
}

// // starting relative paths with super
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }
//
// making struct and enum public

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

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
