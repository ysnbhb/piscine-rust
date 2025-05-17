pub fn num_to_ordinal(x: u32) -> String {
    if x >= 10 && x <= 20 {
        return x.to_string() + &String::from("th");
    }
    if x % 10 == 1 {
        return x.to_string() + &String::from("st");
    } else if x % 10 == 2 {
        return x.to_string() + &String::from("nd");
    } else if x % 10 == 3 {
        return x.to_string() + &String::from("rd");
    }
    x.to_string() + &String::from("th")
}
