use std::iter::Peekable;

pub fn tokenize<'a>(
    lines: &'a mut Peekable<std::str::Lines>,
    errmsg: &'a str,
) -> std::str::Split<'a, char> {
    lines.next().expect(errmsg).split(' ')
}

pub fn consume<T: std::cmp::PartialEq, U: std::iter::Iterator + Iterator<Item = T>>(
    token: &mut Peekable<U>,
    expected: T,
) -> bool {
    if token.peek() == Some(&expected) {
        token.next();
        true
    } else {
        false
    }
}

pub fn expect<
    T: std::cmp::PartialEq + std::fmt::Display + Copy,
    U: std::iter::Iterator + Iterator<Item = T>,
>(
    token: &mut Peekable<U>,
    expected: T,
) {
    if token.next() != Some(expected) {
        panic!("dtb parse error! ('{}' not found)", expected);
    }
}
