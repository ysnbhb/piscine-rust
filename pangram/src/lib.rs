use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut chars_seen: HashSet<char> = HashSet::new();

    for c in s.to_ascii_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            chars_seen.insert(c);
        }
    }

    chars_seen.len() == 26
}
