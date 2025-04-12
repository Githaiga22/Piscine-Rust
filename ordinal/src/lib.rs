pub fn num_to_ordinal(x: u32) -> String {
    let mut s = x.to_string();

    if let 11..=13 = x % 100{
        s.push_str("th");
    } else {
        match x % 10 {
            1 => s.push_str("st"),
            2 => s.push_str("nd"),
            3 => s.push_str("rd"),
            _ => s.push_str("th"),
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
