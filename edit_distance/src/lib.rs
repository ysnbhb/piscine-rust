use std::cmp::min;
pub fn edit_distance(source: &str, target: &str) -> usize {
    let t = target.len() + 1;
    let s = source.len() + 1;
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let mut d2 = vec![vec![0; s]; t];
    for i in 0..t {
        for j in 0..s {
            if i == 0 {
                d2[i][j] = j;
            } else if j == 0 {
                d2[i][j] = i;
            } else if target_chars[i - 1] == source_chars[j - 1] {
                d2[i][j] = d2[i - 1][j - 1];
            } else {
                d2[i][j] = 1 + min(min(d2[i][j - 1], d2[i - 1][j]), d2[i - 1][j - 1]);
            }
        }
    }
    d2[t - 1][s - 1]
}
