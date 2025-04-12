pub fn score(s: &str) -> u32 {
    let mut scr: u32 = 0;
    let s1 = s.to_lowercase();

    for c in s1.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' |  's' | 't' => scr += 1,
            'd' | 'g' => scr += 2,
            'b' | 'c' | 'm' | 'p' => scr += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => scr += 4,
            'k' => scr += 5,
            'j' | 'x' => scr += 8,
            'q' | 'z' => scr += 10,
            _ => scr += 0,
        }
    }

    scr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("ã ê Á?"), 0);
        assert_eq!(score("ThiS is A Test"), 14);
    }
}
