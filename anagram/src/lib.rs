use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    for item in possible_anagrams.iter() {
        if is_anagram(word, item) && !is_same_word(word, item) {
            result.insert(item);
        }
    }

    result
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let word_sorted: String = get_sorted(word).iter().collect();
    let possible_anagram_sorted: String = get_sorted(possible_anagram).iter().collect();

    word_sorted == possible_anagram_sorted
}

fn is_same_word(word: &str, possible_anagram: &str) -> bool {
    word.to_lowercase().as_str() == possible_anagram.to_lowercase().as_str()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.to_lowercase().chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}