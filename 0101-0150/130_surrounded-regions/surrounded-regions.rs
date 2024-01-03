
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }
        
        let rows = board.len();
        let cols = board[0].len();
        
        // 遍历第一列和最后一列，将与边界相连的'O'标记为特殊字符'#'
        for i in 0..rows {
            Self::dfs(board, i, 0);
            Self::dfs(board, i, cols - 1);
        }
        
        // 遍历第一行和最后一行，将与边界相连的'O'标记为特殊字符'#'
        for j in 0..cols {
            Self::dfs(board, 0, j);
            Self::dfs(board, rows - 1, j);
        }
        
        // 遍历整个矩阵，将剩余的'O'替换为'X'，将特殊字符'#'恢复为'O'
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == '#' {
                    board[i][j] = 'O';
                }
            }
        }
    }
    
    fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
        let rows = board.len();
        let cols = board[0].len();
        
        if row >= rows || col >= cols || board[row][col] != 'O' {
            return;
        }
        
        board[row][col] = '#';
        
        // 上下左右四个方向进行深度优先搜索

        Self::dfs(board, row - 1, col);
        Self::dfs(board, row + 1, col);
        Self::dfs(board, row, col - 1);
        Self::dfs(board, row, col + 1);
    }
}
