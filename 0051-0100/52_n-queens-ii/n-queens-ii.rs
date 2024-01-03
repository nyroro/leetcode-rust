
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        Self::backtrack(&mut board, 0, &mut count);
        count

    }
    
    fn backtrack(board: &mut Vec<Vec<char>>, row: usize, count: &mut i32) {
        if row == board.len() {
            *count += 1;
            return;
        }
        
        let n = board.len();
        for col in 0..n {
            if Self::is_valid(board, row, col) {
                board[row][col] = 'Q';
                Self::backtrack(board, row + 1, count);
                board[row][col] = '.';
            }
        }
    }
    
    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        let n = board.len();
        
        // Check if there is a queen in the same column

        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        
        // Check if there is a queen in the upper left diagonal

        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        
        // Check if there is a queen in the upper right diagonal

        let mut i = row as i32 - 1;
        let mut j = col as i32 + 1;
        while i >= 0 && j < n as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        
        true

    }
}
