//标识b生命周期至少与a一致
struct MySmartPointer<'a,'b:'a>{
    data: &'b str,
    test: &'a str
}

impl <'a,'b:'a> Drop for MySmartPointer<'a,'b>{
    fn drop(&mut self) {
        println!("my smart pointer {} has been dropping", self.data);
    }
}

#[test]
fn test_drop_1(){
    let ta = String::from("a");
    let tb = "b";
    let a = MySmartPointer { data: &ta[..] ,test:&ta[..] };
    let b = MySmartPointer { data: tb ,test:tb};
    println!("MySmartPointer created.");
    drop(a);
    println!("MySmartPointer dropped before the end of main.");
}