pub fn scytale_cipher(message: String, i: u32) -> String {
    let len = (message.len() as f64 / i as f64).ceil() as usize;
    let mut grid = vec![vec![' '; i as usize]; len as usize];
    for (j, c) in message.chars().enumerate() {
        let row = j / i as usize;
        let col = j % i as usize;
        grid[row][col] = c
    }
    let mut res = String::new();
    for r in 0..i {
        for j in 0..len {
            res.push(grid[j][r as usize]);
        }
    }

    res[0..message.len()-1].to_string()
}
