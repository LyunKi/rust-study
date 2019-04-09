#[derive(Debug)]
enum UsState {
    A(String),
    B(String),
}

enum Coin {
    C,
    D(UsState),
    E,
}

fn main() {
    let a = Coin::D(UsState::B(String::from("b")));
    match a {
        Coin::C => println!("coin c"),
        Coin::E => println!("coin e"),
        Coin::D(d) => println!("{:#?}", d),
    };

    if let Coin::C = a {
        println!("coin c")
    } else if let Coin::D(e) = a {
        println!("coin d,{:#?}", e);
    } else {
        println!("coin other")
    }
}
