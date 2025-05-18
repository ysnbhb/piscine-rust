pub fn spell(n: u64) -> String {
    if n == 1_000_000 {
        return "one million".to_string();
    } else if n == 0 {
        return "zero".to_string();
    }

    let mut result = String::new();
    let mut n = n;
    if n >= 1_000 {
        let thousands = n / 1_000;
        result.push_str(&spell(thousands));
        result.push_str(" thousand");
        n %= 1_000;
        if n > 0 {
            result.push(' ');
        }
    }

    if n >= 100 {
        let hundreds = n / 100;
        result.push_str(&digit_spell(hundreds as u8));
        result.push_str(" hundred");
        n %= 100;
        if n > 0 {
            result.push(' ');
        }
    }

    if n >= 20 {
        let tens = n / 10;
        result.push_str(&tens_spell(tens as u8));
        n %= 10;
        if n > 0 {
            result.push('-');
        }
    }

    if n > 0 && n < 20 {
        result.push_str(&digit_spell(n as u8));
    }

    result.trim().to_string()
}

fn tens_spell(n: u8) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string(),
    }
}

fn digit_spell(dig: u8) -> String {
    match dig {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}
