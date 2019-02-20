#[macro_export]
macro_rules! print_result {
    ($e:expr) => {
        println!("{:?}={:?}",stringify!($e),$e);
    };
}