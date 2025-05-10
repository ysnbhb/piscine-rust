pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let orgin = c;
    let exp = c as f64;
    let abs = c as f64;

    (orgin, exp.exp(), (abs.abs()).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for i in a.split(" ") {
        let num = i.parse::<f32>().unwrap().exp();
        res.push_str(&num.to_string());
        res.push(' ');
    }
    res.remove(res.len() - 1);
    (a.clone(), res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec: Vec<f64>  = Vec::new();
    for i in b.clone() {
        let abs = i as f64;
        vec.push(abs.abs().ln());
    }
    (b  , vec)
}
