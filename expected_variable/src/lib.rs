use case::{CaseExt, Case};
use std::cmp;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is camelCase or snake_case
    if !compared.is_camel_case() && !compared.is_snake_case() {
        return None;
    }

    // Calculate edit distance (you must define or already have this function)
    let distance = edit_distance(compared.to_lowercase().as_str(), expected.to_lowercase().as_str());

    let expected_len = expected.len();
    if expected_len == 0 {
        return None;
    }

    let similarity = 100 - ((distance * 100) / expected_len);

    if similarity >= 50 {
        Some(format!("{}%", similarity))
    } else {
        None
    }
}

// Damerau-Levenshtein distance (basic edit distance function)
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
                1 + cmp::min(cmp::min(costs[j], costs[j + 1]), last_cost)
            };
            last_cost = costs[j + 1];
            costs[j + 1] = new_cost;
        }
    }

    costs[b.len()]
}
