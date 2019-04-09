#[derive(Copy)]
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(name: String, email: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut name = String::from("a");
    let user1 = build_user(name, String::from("b"));
    //    name.push_str("2"); 因为name的所有权被转移到user1中了，除非name拥有实现了Copy trait的类型
    println!("{}", user1.name);
    let mut user2 = User {
        active: false,
        ..user1
    };
    println!("{}", user2.active);
    user2.active = true;
    println!("{}", user2.active);

    let point1 = Point(0, 0);
    println!("{}", point1.0);

    let rectangle1 = Rectangle {
        width: 20,
        height: 10,
    };
    println!(
        "the area of rectangle {:#?} is {}",
        rectangle1,
        rectangle1.area()
    );
    let rectangle2 = Rectangle::square(10);
    println!(
        "rectangle1 can holed rectangle2? the result is {}",
        rectangle1.can_hold(&rectangle2)
    );
}
