use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Iterator;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split_terminator(|c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('\''))
        .sorted()
        .group_by(|x| x.replace(|c: char| c != '\'' && !c.is_alphanumeric(), ""))
        .into_iter()
        .map(|(w, g)| (w, g.count() as u32))
        .collect()
    /*
    for (word, group) in &words
        .to_lowercase()
        .split_terminator(|c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('\''))
        .sorted()
        .group_by(|x| x.replace(|c: char| c != '\'' && !c.is_alphanumeric(), ""))
    {
        result.insert(word, group.count() as u32);
    }
    result
    */
}
