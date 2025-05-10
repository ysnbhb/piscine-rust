
pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X' , table) || horizontal('X' , table) || vertical('X'  ,table) {
       return  "player X won".to_string();
    }
    if diagonals('O' , table) || horizontal('O' , table) || vertical('O'  ,table) {
       return  "player O won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let n = table.len();
    if (0..n).all(|i| table[i][i] == player) {
        return true;
    }
    if (0..n).all(|i| table[i][n - i - 1] == player) {
        return true;
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|c| c.iter().all(|&v| v == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let n = table.len();
    (0..n).any(|c| (0..n).all(|f| table[f][c] == player))
}
