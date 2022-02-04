use std::iter::Peekable;

fn tokenize<'a>(lines: &'a mut Peekable<std::str::Lines>, errmsg: &'a str)
    -> std::str::Split<'a, char> {
    lines 
        .next()
        .expect(errmsg)
        .trim_left() // remove indent
        .split(' ')
}

pub fn tokenize_node<'a>(lines: &'a mut Peekable<std::str::Lines>)
    -> std::str::Split<'a, char> {
    let tokens = tokenize(lines, "node is invalid");

    tokens
}

pub fn tokenize_prop<'a>(lines: &'a mut Peekable<std::str::Lines>)
    -> std::str::Split<'a, char> {
    let tokens = tokenize(lines, "property is invalid");

    tokens
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
