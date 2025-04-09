use std::collections::HashMap;

pub const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut hmap = HashMap::new();

    for &word in words { 
        *hmap.entry(word).or_insert(0) += 1; 
    }

    hmap
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let mut hmap = HashMap::new();
        let frequency_count = word_frequency_counter(&words);
        hmap.insert("tests", 1);
        hmap.insert("with", 1);
        hmap.insert("this", 2);
        hmap.insert("it", 1);
        hmap.insert("enough", 1);
        hmap.insert("is", 2);
        hmap.insert("but", 1);
        hmap.insert("sentence", 1);
        hmap.insert("only", 1);
        hmap.insert("basic", 3);
        hmap.insert("again", 1);
        hmap.insert("for", 1);
        hmap.insert("be", 1);
        hmap.insert("once", 1);
        hmap.insert("very", 2);
        hmap.insert("should", 1);
        hmap.insert("few", 1);
        hmap.insert("a", 2);
        hmap.insert("repetitions.", 1);
        assert_eq!(frequency_count, hmap);
        assert_eq!(nb_distinct_words(&frequency_count), 20);
    }
}