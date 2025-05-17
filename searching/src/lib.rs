pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, v) in array.iter().enumerate() {
        if *v == key {
            return Some(i);
        }
    }
    None
}
