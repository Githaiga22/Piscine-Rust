pub fn talking(text: &str) -> &str {
    if let true = allcaps(text) {
        match text.chars().last() {
            Some('?') => "Quiet, I am thinking!",
            _ => "There is no need to yell, calm down!",
        }
    } else if let true = text.trim().is_empty() {
        "Just say something!"
    } else {
        match text.chars().last() {
            Some('?') => "Sure.",
            _ => "Interesting",
        }
    }
}

fn allcaps(text: &str) -> bool {
    let mut caps = String::from("");
    let mut smals = String::from("");
    for c in text.chars() {
        if c >= 'A' && c <= 'Z' {
            caps.push(c)
        } else if c >= 'a' && c <= 'z' {
            smals.push(c)
        }
    }

    if smals.len() == 0 && caps.len() != 0 {
        return true;
    }

    false
}