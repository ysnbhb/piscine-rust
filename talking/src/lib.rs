pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }else if is_upp(text) && !text.ends_with('?') {
        return "There is no need to yell, calm down!";
    }else if is_upp(text) && text.ends_with('?') {
        return "Quiet, I am thinking!";
    }else if !is_upp(text) && text.ends_with('?') {
        return "Sure.";
    }else {
        return "Interesting";
    }
}
fn is_upp(ss : &str) -> bool {
    let cc = ss.chars().any(|b| b.is_alphabetic());
    cc && ss == ss.to_ascii_uppercase()
}