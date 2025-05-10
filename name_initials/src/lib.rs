pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_names = Vec::new();
    for name in names {
        let mut new_name = String::new();
        for i in name.chars() {
            if i.is_ascii_uppercase() {
                new_name.push_str(&i.to_string());
                new_name.push_str(". ");
            }
        }
        new_name.remove(new_name.len() - 1);
        new_names.push(new_name.clone());
    }
    new_names
}
