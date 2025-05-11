pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
   let new  = input.split_at(1);
   new.0.to_uppercase() + &new.1.to_lowercase()
}

pub fn title_case(input: &str) -> String {
    let mut new_strin = Vec::new();
    for i in input.split(" "){
        new_strin.push(capitalize_first(i));
    }
    new_strin.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut new_string= String::new();
    for i in input.chars() {
        new_string.push(if i.is_uppercase()  {
            i.to_ascii_lowercase()
        }else {
            i.to_ascii_uppercase()
        });
    }
    new_string
}
