use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    if s.trim().is_empty() {
        return false;
    }
    for i in s.split_ascii_whitespace() {
        let mut res = false;
        let mut mp: HashMap<char, bool> = HashMap::new();
        for j in i.chars() {
            res = *mp.get(&j).unwrap_or(&false);
            if res {
                return false;
            }
            mp.insert(j, true);
        }
    }
    true
}
