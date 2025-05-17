pub fn pig_latin(text: &str) -> String {
    let mut res = String::from(text);
    let i = "aeiou";
    for j in text.chars() {
        if res.starts_with("qu") {
            res = res.chars().skip(2).collect();
            res.push_str("qu");
            break;
        }
        if i.contains(j) {
            break;
        }
        let c = res.remove(0);
        res.push(c);
    }

    res + "ay"
}
