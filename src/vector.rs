pub fn test_vector() {
    let mut a = vec![1, 2, 3, 4];
    for i in &mut a {
        *i += 1;
    }
    for j in &a {
        println!("{}", j);
    }
}
