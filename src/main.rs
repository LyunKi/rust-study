//mod guess;
//mod vector;
//mod string;
//mod declare_macro;
mod class_macro;

trait PrintSelf{
    fn print_self(&self) ;
}

impl PrintSelf for String{
    fn print_self(&self) {
        println!("{}", self);
    }
}
fn main() {
    //    guess::guess_game();
    //    vector::test_vector();
//    string::children::test_string();
//    print_result!({
//        let x = 1u32;
//        x * x + 2 * x - 1
//    });
    class_macro::greet();
    String::from("a").print_self();
}

