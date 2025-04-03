pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return String::from("player X won");
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return String::from("player O won");
    }
    return String::from("tie");
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    // Check secondary diagonal (top-right to bottom-left)
    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in 0..3 {
        if table[row][0] == player && 
           table[row][1] == player && 
           table[row][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && 
           table[1][col] == player && 
           table[2][col] == player {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

        assert_eq!(tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']]), "tie");
        assert_eq!(tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']]), "player O won");
        assert_eq!(tic_tac_toe(diag),"player X won");
    }
}