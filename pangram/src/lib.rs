use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    if s.trim().is_empty() {
        return false;
    }
    let mut mp: HashMap<char, isize> = HashMap::new();
    for i in s.to_ascii_lowercase().chars() {
        if i <= 'a' && i >= 'z' {
            mp.insert(i, 0);
        }
    }
    mp.len() == 26
}
