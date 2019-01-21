pub fn largest<'a, T>(a: &'a T, b: &'a T) -> &'a T
    where T: PartialOrd {
    if a > b {
        a
    }else {
        b
    }
}

pub struct AStructure(pub String);