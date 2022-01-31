use std::iter::Peekable;

//pub fn consume<T: std::cmp::PartialEq + std::fmt::Display + Copy>
pub fn consume<T: std::iter::Iterator + std::cmp::PartialEq + Iterator<Item = T>>
    (token: &mut Peekable<T>, expected: &T) -> bool {
    if token.peek() == Some(expected) {
        token.next();
        true
    } else {
        false
    }
}

pub fn expect<T: std::cmp::PartialEq + std::fmt::Display + Copy>
    (token: Option<T>, expected: T) {
    if token != Some(expected) {
        panic!("dtb parse error! ('{}' not found)", expected);
    }
}
