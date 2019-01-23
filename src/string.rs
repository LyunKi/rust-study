pub mod children {
    pub fn test_string() {
        let a = "n";
        let mut b = a;
        b = "3";
        let mut c = String::from("123");
        c.push_str("world");
        println!("{}", c);
    }
}
