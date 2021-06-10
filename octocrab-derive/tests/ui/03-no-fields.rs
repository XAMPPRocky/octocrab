#![allow(dead_code)]

#[derive(octocrab_derive::Builder)]
struct Test<'a>(&'a str);

fn main() {}
