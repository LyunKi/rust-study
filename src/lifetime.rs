use std::fmt::Display;

pub fn largest<'a, T>(a: &'a str, b: &'a str, c: T) -> &'a str
where
    T: Display,
{
    println!("{}", c);
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

pub struct AStructure(pub String);

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}

fn do_it<'a>(data: &'a (u8, u16)) -> &'a u8 {
    &data.0
}

struct Inspector<'a>(&'a u8, &'static str);

impl<'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("再过{}天就退休", self.1)
    }
}

use std::marker;

struct MyVec<T> {
    data: *const T, // *const是可变的！
    len: usize,
    cap: usize,
    //如果结构体中实际没用到T，但又需要让编译器明白，要能销毁T后再销毁MyVec
    _marker: marker::PhantomData<T>,
}

#[test]
fn test_pointer_box_4() {
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    assert_eq!(clo.call(), &0);
}

#[test]
fn may_dangle() {
    let (a, b): (Inspector, Box<u8>);
    b = Box::new(2);
    println!("{}", b);
    //cant compile
    //    a = Inspector(&b, "3");
}
