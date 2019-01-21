pub trait Summary {
    fn get_name(&self) -> String {
        String::from("unknown")
    }
    fn summarize(&self) {
        println!("from {}", self.get_name());
    }
}

pub struct Tweet {
    pub a: String,
}

pub struct NewArticle {
    pub a: String,
}

impl Summary for Tweet {
    fn get_name(&self) -> String {
        String::from("tweet")
    }
}

impl Summary for NewArticle {}

pub struct Container<T: Summary> {
    pub content: T,
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: Clone, U: Clone> Point<T, U> {}

pub fn back_input<T>(input: T) -> T
where
    T: Clone,
{
    input
}
