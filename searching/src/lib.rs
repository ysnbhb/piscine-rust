pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for i in array {
        if *i == key {
            return  Some(*i as usize ) ;
        }
    }
    None
}