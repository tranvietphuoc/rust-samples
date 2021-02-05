#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // print_user is method of struct User
    fn print_user(&self) {
        println!("user name: {}, user email: {}", self.username, self.email);
    }
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn print_rect(&self) {
        println!("{}, {}", self.width, self.height);
    }

    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u64) -> Rectangle {
        // change the ownership here
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("phuoc.finn@gmail.com"),
        username: String::from("Tran Viet Phuoc"),
        active: true,
        sign_in_count: 1,
    };

    // creating an instance from other instance with struct update synctax
    let user2 = User {
        email: String::from("test@gmail.com"),
        username: String::from("test"),
        ..user1  //.. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance
    };

    println!("user2: {:?}", user2);
    println!("print out user using method");
    user1.print_user();
    println!("print out user using #[derive(DeBug)] marco");
    println!("{:?}", user1);

    //
    let rect1 = Rectangle {
        width: 30,
        height: 5,
    };

    //
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    rect1.print_rect();
    println!("Area {}", rect1.area());
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    // associated function
    let square = Rectangle::square(20);
    square.print_rect();
    println!("hello");
}
