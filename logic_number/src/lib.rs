pub fn number_logic(num: u32) -> bool {
    let mut res = 0;
    let len = num.to_string().len();
    for i in num.to_string().chars() {
        let num = i.to_string().parse::<u32>().unwrap();
        res += num.pow(len as u32);
    }
    res == num
}
