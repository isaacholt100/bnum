use bint::{U128, BUint};
use lazy_static::lazy_static;

fn main() {
    println!("{}", u8::from_str_radix("a", 10).unwrap_err());
}

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref CLONE: HashMap<u32, &'static str> = HASHMAP.clone();
}