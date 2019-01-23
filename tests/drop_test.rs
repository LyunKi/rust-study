struct MySmartPointer<'a>{
    data: &'a str
}

impl <'a> Drop for MySmartPointer<'a>{
    fn drop(&mut self) {
        println!("my smart pointer {} has been dropping", self.data);
    }
}

#[test]
fn test_drop_1(){
    let a = MySmartPointer { data: "a" };
    let b = MySmartPointer { data: "b" };
    println!("MySmartPointer created.");
    drop(a);
    println!("MySmartPointer dropped before the end of main.");
}