pub fn factorial(num: u64) -> u64 {
    if num ==0 {
        return 0;
    }else {
        let mut i = 1 ; 
        let mut cp = num;
        while cp !=0 {
            i*= cp ;
            cp -=1 ;
        }
        return i ;
    }
}

