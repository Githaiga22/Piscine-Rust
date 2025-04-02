pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val); // Adds the new element to the end of the vector
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index] // Returns a reference to the string at the given index
}
