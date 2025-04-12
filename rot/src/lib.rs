pub fn rotate(input: &str, key: i8) -> String {
    let mut s = String::new();
    let i;

    if key < 0 {
        i = 26 + key;
    } else {
        i = key;
    }

    for c in input.chars() {
        match c {
            'a'..='z' => s.push(((((c as u8 - 'a' as u8) as i8 + i) % 26) as u8 + 'a' as u8) as char),
            'A'..='Z' => s.push(((((c as u8 - 'A' as u8) as i8 + i) % 26) as u8 + 'A' as u8) as char),
            _ => s.push(c)

        }
    }
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rotate("a", 25), "z");
        assert_eq!(rotate("m", 0), "m");
        assert_eq!(rotate("m", 13), "z");
        assert_eq!(rotate("a", 15), "p");
        assert_eq!(rotate("MISS", 5), "RNXX");
        assert_eq!(rotate("Testing", -14), "Fqefuzs");
    }
}
