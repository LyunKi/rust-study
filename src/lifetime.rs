use std::fmt::Display;

pub fn largest<'a, T>(a: &'a str, b: &'a str,c:T) -> &'a str
    where T: Display {
    println!("{}", c);
    if a.len() > b.len() {
        a
    }else {
        b
    }
}

pub struct AStructure(pub String);