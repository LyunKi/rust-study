extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret num is {}", secret_number);
    loop {
        println!("print a number what you guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail to read");
        println!("I guess it's {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        };
    }
}
