pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '-' if !result.is_empty() => {
                result.pop(); // Simulate backspace
            }
            '+' => {
                // Simulate delete: remove next character if exists
                if i + 1 < chars.len() {
                    chars.remove(i + 1);
                }
            }
            _ => {
                result.push(chars[i]); // Keep valid characters
            }
        }
        i += 1;
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let parts: Vec<&str> = expr.split(|c| c == '+' || c == '-').collect();
        if parts.len() == 2 {
            let left: i32 = parts[0].parse().unwrap_or(0);
            let right: i32 = parts[1].parse().unwrap_or(0);
            let result = if expr.contains('+') {
                left + right
            } else {
                left - right
            };
            *expr = result.to_string();
        }
    }
}
