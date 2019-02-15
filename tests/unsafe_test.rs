extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn test_unsafe_1(){
    let mut a = 5;
    let b = &a as *const i32;
    let c = &mut a as *mut i32;
    unsafe {
        assert_eq!(5, *b);
        assert_eq!(5, *c);
    }
}

#[test]
fn test_unsafe_2() {
    unsafe {
        assert_eq!(3, abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}