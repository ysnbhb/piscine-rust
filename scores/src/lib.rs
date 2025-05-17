pub fn score(s: &str) -> usize {
    let mut res: usize = 0;
    for i in s.to_lowercase().chars() {
        if ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'].contains(&i) {
            res += 1;
        } else if ['D', 'G'].contains(&i) {
            res += 2;
        }else if ['B','C', 'M', 'P'].contains(&i) {
              res += 3;
        }else if ['F', 'H', 'V', 'W','Y'].contains(&i) {
              res += 4;
        }else if ['K'].contains(&i) {
              res += 5;
        }else if  ['J' ,'X'].contains(&i){
             res += 8;
        }else if  ['Q' ,'Z'].contains(&i){
             res += 10;
        }
    }
    res
}
