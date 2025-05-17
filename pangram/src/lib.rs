use std::{collections::HashMap};

pub fn is_pangram(s: &str) -> bool {
    if s.trim().is_empty() {
        return false;
    }
    let mut mp: HashMap<char, isize> = HashMap::new();
    for i in s.as_bytes() {
        if i.is_ascii_alphabetic() {
            mp.insert(*i as char, 0);
        }
    }
    mp.len() == 26
}
