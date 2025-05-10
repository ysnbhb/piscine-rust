pub fn delete_and_backspace(s: &mut String) {
    let mut new_str = String::new();
    let mut count = 0;
    for chare in s.chars() {
        if chare != '-' && chare != '+' && count == 0 {
            new_str.push(chare);
        } else if chare == '-' {
            new_str.pop();
        } else if chare == '+' {
            count += 1;
        } else {
            count -= 1
        }
    }
    *s = new_str
}

pub fn do_operations(v: &mut [String]) {
    for num in v.iter_mut() {
        let mut plus = false;
        let vec: Vec<String> = if num.contains('-') {
            num.split("-").map(|s| s.to_string()).collect()
        } else {
            plus = true;
            num.split('+').map(|s| s.to_string()).collect()
        };
        let a = vec[0].parse::<isize>().unwrap();
        let b = vec[1].parse::<isize>().unwrap();
        let res = if plus { a + b } else { a - b };
        *num = res.to_string();
    }
}

