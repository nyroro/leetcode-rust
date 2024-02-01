
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board)
    }
    
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for num in ['1', '2', '3', '4', '5', '6', '7', '8', '9'].iter() {
                        if Self::is_valid(board, i, j, *num) {
                            board[i][j] = *num;
                            if Self::solve(board) {
                                return true;
                            }
                            board[i][j] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true

    }
    
    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        for i in 0..9 {
            if board[i][col] == num {
                return false;
            }
            if board[row][i] == num {
                return false;
            }
            if board[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] == num {
                return false;
            }
        }
        true

    }
}
