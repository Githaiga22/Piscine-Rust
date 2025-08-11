pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|num_str| {
            let multiplier = if num_str.ends_with('k') { 1000.0 } else { 1.0 };

            // Remove 'k' if present
            let clean_str = num_str.trim_end_matches('k');

            // Parse as f64 to handle decimals
            let value: f64 = clean_str.parse().unwrap_or(0.0);

            // Multiply and convert to u32
            let final_value = (value * multiplier) as u32;

            Box::new(final_value)
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|boxed| *boxed).collect()
}
