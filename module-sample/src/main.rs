// use module_sample::back_of_house;  // to use module from lib.rs, we use package::module
use module_sample::back_of_house;

fn main(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} please", meal.toast);
}
