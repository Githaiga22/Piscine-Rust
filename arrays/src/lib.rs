pub fn sum(a: &[i32]) -> i32 {
    // a.iter().sum()
    let mut sum:i32 = 0;
    for i in a {
        sum += i;
    }

    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    // [132; 32]
    let mut res:[i32; 32]=[0; 32];
    for i in  0..32 {
        res[i]=10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        let c = [10; 32];
        assert_eq!(sum(&a), 55);
        assert_eq!(sum(&b), 50);
        assert_eq!(thirtytwo_tens(), c);
    }
}
