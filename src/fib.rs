fn main() {
    while_fib(200);
}

fn while_fib(n: i32) {
    let mut i = 1;
    let mut j = 1;
    println!("{}", i);
    println!("{}", j);
    while {
        j = i + j;
        i = j - i;
        j
    } <= n {
        println!("{}", j);
    }
}