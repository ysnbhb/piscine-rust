use std::{collections::HashMap, f32::INFINITY};
pub fn mean(list: &[i32]) -> f64 {
    let all: i32 = list.into_iter().sum();
    all as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut new_list = list.to_vec();
    new_list.sort();
    new_list[new_list.len() / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &word in list {
        *map.entry(word).or_insert(0) += 1;
    }
    bigger(map)
}

pub fn bigger(h: HashMap<i32, i32>) -> i32 {
    let mut big = -INFINITY as i32;
    let mut value = 0;
    for (v, num) in h {
        big = if num > big {
            value = v;
            num
        } else {
            big
        };
    }
    value
}
