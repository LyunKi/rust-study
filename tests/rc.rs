use std::rc::Rc;

enum List<T>{
    Cons(T,Rc<List<T>>),
    Nil
}

#[test]
fn test_rc_1(){
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