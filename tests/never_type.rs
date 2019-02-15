fn never_back ()->!{
    panic!("123")
}

#[test]
fn test_never_back(){
    never_back();
}