#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res = String::new();
    for i in original.chars() {
        if i.is_ascii_alphabetic() {
            if i.is_ascii_uppercase() {
                res.push(('A' as u8 + ('Z' as u8 - i as u8)) as char);
            } else {
                res.push(('a' as u8 + ('z' as u8 - i as u8)) as char);
            }
        } else {
            res.push(i);
        }
    }

    if res == ciphered.to_string() {
        Ok(())
    } else {
        Err(CipherError { expected: res })
    }
}