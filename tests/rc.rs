use std::rc::Rc;
use std::cell::RefCell;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

#[test]
fn test_rc_1() {
    let a = Rc::new(List::Cons(1, Rc::new(List::Nil)));
    assert_eq!(1, Rc::strong_count(&a));
    let b = List::Cons(2, Rc::clone(&a));
    assert_eq!(2, Rc::strong_count(&a));
    {
        let b = List::Cons(3, Rc::clone(&a));
        assert_eq!(3, Rc::strong_count(&a));
    }
    assert_eq!(2, Rc::strong_count(&a));
}

#[derive(Debug)]
enum MutableList<T> {
    Cons(Rc<RefCell<T>>, Rc<MutableList<T>>),
    Nil,
}

#[test]
fn test_rc_2() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(MutableList::Cons(Rc::clone(&value), Rc::new(MutableList::Nil)));
    let b = MutableList::Cons(Rc::new(RefCell::new(1)), Rc::clone(&a));
    let c = MutableList::Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    //这里能这么写是因为rust的自动解引用功能.
//    *((*value).borrow_mut()) += 10;
    *(value.borrow_mut()) += 10;
    println!("{}", Rc::strong_count(&value));
    println!("{}", Rc::strong_count(&a));
    println!("{:?}", b);
    println!("{:?}", c);
}

#[derive(Debug)]
enum CircularList {
    Cons(i32, RefCell<Rc<CircularList>>),
    Nil,
}

impl CircularList {
    fn tail(&self) -> Option<&RefCell<Rc<CircularList>>> {
        match self {
            CircularList::Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[test]
fn test_crc_1() {
    use CircularList::{Cons, Nil};
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    };
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
// 取消如下行的注释来观察引用循环;
// 这会导致栈溢出
//     println!("a next item = {:?}", a.tail());
}