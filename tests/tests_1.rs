extern crate rust_study;
mod common;

#[test]
fn back_one_test() {
    assert_eq!(common::back_one(), 1);
}

#[test]
fn back_two_test(){
    assert_eq!(rust_study::back_two(), 2);
}

#[test]
fn life_test(){
    let a = "ab";
    let b = "a";
    let c = "compare beginning";
    assert_eq!(rust_study::largest(a,b,c),a)
}

#[test]
fn create_new_structure() {
    use self::rust_study::AStructure;
    let c = AStructure(String::from("123"));
    println!("{}", c.0);
}

#[test]
fn back_result_test()->Result<(), String>{
    if 1 + 1 == 3{
        Ok(())
    }else {
        Err(String::from("1+1!=3"))
    }
}

#[test]
#[should_panic]
fn panic_test(){
   assert_eq!(1,2)
}

