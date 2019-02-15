#[test]
fn while_let_test() {
    let mut a = vec![1, 2, 3, 4];
    while let Some(v) = a.pop() {
        println!("{}", v);
    }
}

#[test]
fn fn_match_test_1() {
    let a = |&(x, y): &(i32, i32)| -> (i32, i32){
        (x, y)
    };
    let point = (1, 2);
    assert_eq!(point, a(&point));
}
#[test]
fn fn_match_test_2() {
    let a = Some(String::from("hello"));
    match a {
        Some(a) => println!("matched,{:?}",a),
        _ => println!("something err"),
    };
//    println!("you can't use a:{:?} again",a)
}