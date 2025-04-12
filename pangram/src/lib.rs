use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map = HashMap::new();

    for c in s.to_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
    }

    match map.len() {
        26 => true,
        _  => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_pangram("Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.");
        assert_eq!(result, true);
    }
}
