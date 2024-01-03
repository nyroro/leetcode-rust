
impl Solution {
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        let n = board.len();
        let m = board[0].len();
        let mut transposed_board = vec![vec![' '; n]; m];
        
        for i in 0..n {
            for j in 0..m {
                transposed_board[j][i] = board[i][j];
            }
        }
        
        let mut result = false;
        
        for i in 0..n {
            result = result || Self::check_word(&board[i], &word);
            result = result || Self::check_word(&board[i].iter().rev().cloned().collect(), &word);
        }
        
        for i in 0..m {
            result = result || Self::check_word(&transposed_board[i], &word);
            result = result || Self::check_word(&transposed_board[i].iter().rev().cloned().collect(), &word);
        }
        
        result

    }
    
    fn check_word(row: &Vec<char>, word: &String) -> bool {
        let n = row.len();
        let m = word.len();
        
        if n < m {
            return false;
        }
        
        for i in 0..=n - m {
            let mut valid = true;
            for j in 0..m {
                if row[i + j] != ' ' && row[i + j] != word.chars().nth(j).unwrap() {
                    valid = false;
                    break;
                }
            }
            if valid && (i == 0 || row[i - 1] == '#') && (i + m == n || row[i + m] == '#') {
                return true;
            }
        }
        
        false

    }
}
