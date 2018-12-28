use std::io;
use rand::Rng;
fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret num is {}", secret_number);
    println!("print a number what you want");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail to read");
    println!("I guess it's {}",guess);
}