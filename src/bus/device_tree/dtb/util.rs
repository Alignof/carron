use std::iter::{SkipWhile, Peekable};

pub fn tokenize<'a, P: for<'r> std::ops::FnMut(&'r &'a str,)>(lines: &'a mut Peekable<std::str::Lines>, errmsg: &'a str)
    -> SkipWhile<std::str::Split<'a, char>, P> {
    lines
        .next()
        .expect(errmsg)
        .split(' ')
        .skip_while(|t| *t == "")
}

pub fn consume<T: std::cmp::PartialEq, U: std::iter::Iterator + Iterator<Item = T>>
    (token: &mut Peekable<U>, expected: T) -> bool {
    if token.peek() == Some(&expected) {
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
