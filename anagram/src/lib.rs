use std::collections::HashSet;
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = word.to_lowercase().chars().sorted().collect::<String>();
    possible_anagrams
        .iter()
        .filter(|s| word.to_lowercase() != s.to_lowercase() && is_anagram_of(s, &sorted_word))
        .copied()
        .collect::<HashSet<&'a str>>()
}

fn is_anagram_of(s: &str, word: &str) -> bool {
    let sorted_s  = s.to_lowercase().chars().sorted().collect::<String>();
    sorted_s == word
}
