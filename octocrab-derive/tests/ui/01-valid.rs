#![allow(dead_code)]

#[derive(octocrab_derive::Builder)]
struct Test<'a, 'b, 'c> {
    a: &'a str,
    b: Option<&'b str>,
    c: std::option::Option<&'c str>,
    d: Option<bool>,
}

fn main() {}
