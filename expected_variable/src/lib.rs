pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if !is_snake_case(compared) && !is_camel_case(compared) {
        return None;
    }

    let distance = edit_distance(
        &compared.to_lowercase(),
        &expected.to_lowercase()
    );

    let expected_len = expected.len() as i32;
    let distance = edit_distance(&compared.to_lowercase(), &expected.to_lowercase()) as i32;
    
    if expected_len == 0 {
        return None;
    }
    
    let similarity = 100 - ((distance * 100) / expected_len);
    if similarity >= 50 {
        Some(format!("{}%", similarity))
    } else {
        None
    }
    
// Simple check for snake_case
fn is_snake_case(s: &str) -> bool {
    !s.contains(char::is_uppercase) && s.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
}

// Simple check for camelCase (lowerCamel)
fn is_camel_case(s: &str) -> bool {
    !s.contains('_') && s.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or(false)
}

// Edit distance function
fn edit_distance(a: &str, b: &str) -> usize {
    let mut costs = vec![0; b.len() + 1];

    for j in 0..=b.len() {
        costs[j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        let mut last_cost = i;
        costs[0] = i + 1;

        for (j, cb) in b.chars().enumerate() {
            let new_cost = if ca == cb {
                last_cost
            } else {
                1 + std::cmp::min(std::cmp::min(costs[j], costs[j + 1]), last_cost)
            };
            last_cost = costs[j + 1];
            costs[j + 1] = new_cost;
        }
    }

    costs[b.len()]
}
