mod r#trait;
pub mod lifetime;

pub fn back_two()->i32 {
    2
}

pub use lifetime::largest;

pub use lifetime::AStructure;

#[cfg(test)]
mod tests {
    use r#trait::{back_input, NewArticle, Summary};

    #[test]
    fn do_test() {
        let s = NewArticle {
            a: String::from("new article"),
        };
        assert_eq!(s.get_name(), "unknown");
    }

    #[test]
    #[ignore]
    fn test_back() {
        let test = 123;
        assert_eq!(back_input(test), test);
    }

}
