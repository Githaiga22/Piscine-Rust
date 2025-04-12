pub fn pig_latin(text: &str) -> String {
    let mut end = String::new();
    let mut start = String::new();
    let mut found_vowel = false;
    let mut last = text.chars().next();

    for c in text.chars() {
        if (!found_vowel && !is_vowel(c)) || c == 'u' && last == Some('q') {
            end.push(c);
        } else {
            found_vowel = true;
            start.push(c);
        }
        last = Some(c)
    }

    start + &end + "ay"
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay".to_string());
        assert_eq!(pig_latin(&String::from("apple")), "appleay".to_string());
        assert_eq!(pig_latin(&String::from("hello")), "ellohay".to_string());
        assert_eq!(pig_latin(&String::from("square")), "aresquay".to_string());
    }
}
