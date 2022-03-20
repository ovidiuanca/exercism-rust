// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_hash: HashMap<&str, u8> = HashMap::new();
    let mut note_word_hash: HashMap<&str, u8> = HashMap::new();

    let result: bool = false;

    build_hash(magazine, &mut magazine_word_hash);
    build_hash(note, &mut note_word_hash);

    for (word, count) in note_word_hash.iter() {
        if let Some(val) = magazine_word_hash.get(word) {
            if val < count { return false }
        } else {
            return false
        }
    }

    true
}

fn build_hash<'a>(sentence: &[&'a str], hash: &mut HashMap<&'a str, u8>) {
    for word in sentence {
        let word_count = hash.get(word);

        match word_count {
            None => hash.insert(word, 1),
            Some(&count) => hash.insert(word, count + 1)
        };
    }
}
