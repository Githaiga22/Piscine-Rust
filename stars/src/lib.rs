use std::iter;

pub fn stars(n: u32) -> String {
    let s = 2u32.pow(n);
    let str: String = iter::repeat('*').take(s.try_into().unwrap()).collect();
    str
}