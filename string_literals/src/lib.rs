pub fn is_empty(v: &str) -> bool {
    v == ""
}

pub fn is_ascii(v: &str) -> bool {
    for i in v.chars() {
        if i > 127 as char || i < 0 as char {
            return  false;
        }
    } ;
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    let index  = 0;

    for (i , index) in v.chars().enumerate() {
        if index == pat {
            return  i;
        }
    }

    index
}
