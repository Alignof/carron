pub fn consume<T: std::cmp::PartialEq + std::fmt::Display + Copy>(token: Option<T>, expected: T) {
    if token != Some(expected) {
        panic!("dtb parse error! ('{}' not found)", expected);
    }
}
