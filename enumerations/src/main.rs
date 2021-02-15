//
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn substract_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i-1),
    }
}

fn main() {
    let x: i8 = 5;
    let y: Option<i32> = Some(5);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", x);
    println!("{:?}", y);
    println!("{:?}", x);
    println!("{:?}", plus_one(y).unwrap());
    println!("{:?}", substract_one(y).unwrap());
}
