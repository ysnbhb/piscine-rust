pub fn first_subword(mut s: String) -> String {
    let mut new_name = String::new();
    for (index , chare) in s.chars().enumerate(){
        if index ==0 {
            new_name.push(chare);
            continue;
        }
        if chare.is_ascii_uppercase() || chare == '_' {
            break;
        }
        new_name.push(chare);
    }
    new_name
}