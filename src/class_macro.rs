extern crate hello_macro;
extern crate hello_macro_derive;

use self::hello_macro::HelloMacro;
use self::hello_macro_derive::HelloMacro;
use self::proc_macro::TokenStream;

#[derive(HelloMacro)]
struct Pancakes;

pub fn greet() {
    Pancakes::hello_macro();
}

