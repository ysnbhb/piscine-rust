pub fn factorial(num: u64) -> u64 {
    let mut count = 1 ;
    if num == 0 {
        return 1;
    } else {
        for i in 2..num {
            count*=i;
        }
        count
    }
}
