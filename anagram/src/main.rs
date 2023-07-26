use std::collections::{HashSet, HashMap};

fn main() {
    anagrams_for("world", &["hello", "world", "zombies", "pants"]);
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    // Count the occurrences of characters in the word
    let mut word_char_count: HashMap<char, usize> = HashMap::new();
    for character in word.to_lowercase().chars() {
        *word_char_count.entry(character).or_insert(0) += 1;
    }
    let chars = word.chars();
    for & scanning_word in possible_anagrams {
        if scanning_word.trim().len() == word.len() {
            let mut scanning_char_count: HashMap<char, usize> = HashMap::new();
            if scanning_word.to_lowercase().trim() == word.to_lowercase().trim() {
                break;
            }
            // Count the occurrences of characters in the scanning_word
            for character in scanning_word.to_lowercase().chars() {
                *scanning_char_count.entry(character).or_insert(0) += 1;
            }
            if word_char_count == scanning_char_count {
                result.insert(scanning_word);
            }
        }
    }
    result
}