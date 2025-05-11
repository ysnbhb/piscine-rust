pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    chars1.sort();
    let mut chars2: Vec<char> = s2.chars().collect();
    chars2.sort();
    chars1 == chars2
}
