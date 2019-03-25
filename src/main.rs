// enable the await! macro, async support, and the new std::Futures api.
#![feature(await_macro, async_await, futures_api)]
// only needed if we want to manually write a method to go forward from 0.1 to 0.3 future,
// or manually implement a std future (it provides Pin and Unpin):
#![feature(pin)]
// only needed to manually implement a std future:
#![feature(arbitrary_self_types)]

#[macro_use]
extern crate tokio;

//mod guess;
//mod vector;
//mod string;
//mod declare_macro;
//mod class_macro;
//trait PrintSelf {
//    fn print_self(&self);
//}
//
//impl PrintSelf for String {
//    fn print_self(&self) {
//        println!("{}", self);
//    }
//}

mod async_l;

fn main() {
    //    guess::guess_game();
    //    vector::test_vector();
    //    string::children::test_string();

    //    print_result!({
    //        let x = 1u32;
    //        x * x + 2 * x - 1
    //    });

    //    class_macro::greet();

    //    String::from("a").print_self();

    async_l::al_print_sleep();
    async_l::transform_o2n();
    async_l::transform_n2o();
    async_l::execution_concurrently();
}
