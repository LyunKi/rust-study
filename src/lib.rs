mod r#trait;

pub use r#trait::{back_input, NewArticle, Summary};

pub mod closure;

pub mod lifetime;

pub use lifetime::largest;
pub use lifetime::AStructure;

pub fn back_two() -> i32 {
    2
}


pub mod pointer {
    use std::ops::Deref;

    pub fn box_1() {
        let b = Box::new(1);
        println!("box_1 print b : {}", b);
    }
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    pub struct MyBox<T>(T);

    impl<T> MyBox<T>{
        pub fn new(value:T) ->MyBox<T> {
            MyBox(value)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target =T;

        fn deref(&self) -> &<Self as Deref>::Target {
            &self.0
        }
    }
}


#[cfg(test)]
mod tests {
    use self::closure::Counter;
    use super::*;
    use pointer::MyBox;

    #[test]
    fn do_test() {
        let s = NewArticle {
            a: String::from("new article"),
        };
        assert_eq!(s.get_name(), "unknown");
    }

    #[test]
    #[ignore]
    fn test_back() {
        let test = 123;
        assert_eq!(back_input(test), test);
    }

    #[test]
    fn test_move_closure() {
        assert!(closure::move_closure());
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

    #[test]
    fn test_pointer_box_1() {
        pointer::box_1();
    }

    #[test]
    fn test_pointer_box_2() {
        use pointer::List::{Cons, Nil};
        let list = Cons(1,
                        Box::new(Cons(2,
                                      Box::new(Cons(3,
                                                    Box::new(Nil))))));
    }

    #[test]
    fn test_pointer_box_3() {
        let my_box = MyBox::new("a");
        assert_eq!("a", *my_box);
    }

    #[test]
    fn test_pointer_box_4() {
        let my_box = MyBox::new(String::from("ly"));
        fn hello(x:&str) -> String{
            format!("hello,{}", x)
        }
        //这里之所以能通过编译是因为存在了强制解引用的存在，两者等价
        assert_eq!("hello,ly", hello(&my_box));
//        assert_eq!("hello,ly", hello(&(*my_box)[..]));
    }
}
