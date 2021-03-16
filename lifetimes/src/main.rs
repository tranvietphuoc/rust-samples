// examples about lifetime
//

const MAJOR_VER: i32 = 1;
static MINOR_VER: i32 = 0;

fn borrow_it(qty: &str) {
    println!("your money is ${}", qty);
}

fn consume_it(qty: &str) {
    println!("my money is ${}", qty);
}

#[derive(Debug)]
struct Earth {
    location: String,
}

#[derive(Debug)]
struct Dinosaur<'a> {
    // lifetime annotation to specifying Dinosaur is the same with lifetime of Earth
    location: &'a Earth,
    name: String,
}

impl<'a> From<Dinosaur<'a>> for String {
    fn from(d: Dinosaur) -> String {
        format!("{:?}", d)
    }
}

fn main() {
    let money: String = String::from("42");
    borrow_it(money.as_str());
    consume_it(money.as_str());

    let new_york = Earth {
        location: String::from("New York, NY"),
    };
    let t_rex = Dinosaur {
        location: &new_york,
        name: String::from("T. Rex"),
    };

    println!("{}", String::from(t_rex));
}
