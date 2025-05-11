pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return  "".to_string();
    }
   let new  = input.split_at(1);
   new.0.to_uppercase() + &new.1.to_lowercase()
}

pub fn title_case(input: &str) -> String {
    let mut s = String::new();
    let mut boo = false;
    for (i,c) in input.chars().enumerate(){
        if i == 0 || (boo && !c.is_whitespace() ) {
            boo = false;
            if !c.is_uppercase(){
                s.push_str(&c.to_string().to_uppercase());
            } else{
                s.push(c);
            }
        } else if c.is_whitespace() {
            boo = true;
            s.push(c);
        } else {
            s.push(c);
        }
    }
    s
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
