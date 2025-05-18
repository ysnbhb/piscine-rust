pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    let mut res = Vec::<String>::new();
    let mut space_after = 0;
    for letter in 'A'..=c {
        let mut elem = String::new();
        let space_before = c as usize - letter as usize;
        elem.push_str(&" ".repeat(space_before));
        elem.push(letter);

        if letter != 'A' {
            space_after += 1;
            
            elem.push_str(&" ".repeat(space_after * 2 - 1));
            elem.push(letter);
        }
        elem.push_str(&" ".repeat(space_before));
        res.push(elem);
    }
     for letter in ('A'..c).rev() {
        let mut elem = String::new();
        let space_before = c as usize - letter as usize;
        elem.push_str(&" ".repeat(space_before));
        elem.push(letter);

        if letter != 'A' {
            space_after -= 1;
            elem.push_str(&" ".repeat(space_after * 2 - 1));
            elem.push(letter);
        }
        elem.push_str(&" ".repeat(space_before));
        res.push(elem);
    }
    res
}
