use core::f32;
use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().nth(0).unwrap().to_string()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let fnc = self.flags.get(input);
        match fnc {
            Some(f) => {
                let res = f(argv[0], argv[1]);
                match res {
                    Ok(re) => Ok(re),
                    Err(_) => Err("invalid float literal".to_string()),
                }
            }
            None => Err("invalid float literal".to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 = a.parse::<f32>()?;
    let num2 = b.parse::<f32>()?;
    Ok((num1 / num2).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 = a.parse::<f32>()?;
    let num2 = b.parse::<f32>()?;
    Ok((num1 % num2).to_string())
}
