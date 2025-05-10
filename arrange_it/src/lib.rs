pub fn arrange_phrase(phrase: &str) -> String {
    let old_str : Vec<&str> = phrase.split(" ").collect();
    let mut new_arr: Vec<String> = vec!["".to_string(); old_str.len()];
    for i in old_str {
        let mut word = String::new();
        let mut num = 0;
        for j in i.chars() {
            if j.is_numeric(){
                num = j.to_string().parse::<usize>().unwrap();
            }else {
                word.push(j);
            }
        }
        new_arr[num -1] = word.to_string()
    }
    new_arr.join(" ")
}
