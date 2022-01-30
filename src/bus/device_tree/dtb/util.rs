pub fn consume(token: Option<&str>, expected: &str) {
    if token != Some(expected) {
        panic!("dtb parse error! ('{}' not found)", expected);
    }
}
