pub fn number_logic(mut num: u32) -> bool {
    let n = num.to_string().len() as u32;
    let check = num;
    let mut check1 = 0;
    while num > 0 {
        check1 += (num % 10).pow(n.try_into().unwrap());
        num = num / 10;
    }
    check1 == check
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(number_logic(9), true);
        assert_eq!(number_logic(153), true);
        assert_eq!(number_logic(10), false);
        assert_eq!(number_logic(154), false);
    }
}
