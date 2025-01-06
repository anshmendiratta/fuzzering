#![allow(dead_code)]

use internal_macros::fuzzthis;

fn main() {}

#[fuzzthis]
struct Test {
    a: [u8; 2],
}
