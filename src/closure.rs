pub fn move_closure() -> bool {
    let x = String::from("2333");
    let equal_to_x = move |y: String| y == x;
    //    println!(" i can't use x {}", x);
    equal_to_x(String::from("2333"))
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
