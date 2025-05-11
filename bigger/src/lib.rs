use std::{collections::HashMap, f32::INFINITY};
pub fn bigger(h: HashMap<&str, i32>) -> i32 {

    let mut big = -INFINITY as i32;
    for (_ , num) in h {
        big = if num > big {
            num
        }else {
            big
        };
       
    }
    big

}