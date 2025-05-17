pub fn rotate(input: &str, key: i8) -> String {
    let key = ((key % 26) + 26) % 26;
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + key as u8) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + key as u8) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}
