pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut i: usize = 0;

    while i < array.len() {
        if array[i] == key {
            return Some(i);
        }
        i += 1;
    }
    None
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
