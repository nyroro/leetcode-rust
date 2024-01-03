
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
        Self::backtrack(&mut result, &mut board, 0, n as usize);
        result

    }
    
    fn backtrack(result: &mut Vec<Vec<String>>, board: &mut Vec<Vec<char>>, row: usize, n: usize) {
        if row == n {
            let solution: Vec<String> = board.iter().map(|row| row.iter().collect()).collect();
            result.push(solution);
            return;
        }
        
        for col in 0..n {
            if Self::is_safe(board, row, col, n) {
                board[row][col] = 'Q';
                Self::backtrack(result, board, row + 1, n);
                board[row][col] = '.';
            }
        }
    }
    
    fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        
        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        
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
