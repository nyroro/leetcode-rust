
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        
        let mut next_board = vec![vec![0; n]; m];
        
        for i in 0..m {
            for j in 0..n {
                let live_neighbors = Self::count_live_neighbors(board, i, j);
                
                if board[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        next_board[i][j] = 0;
                    } else {
                        next_board[i][j] = 1;
                    }
                } else {
                    if live_neighbors == 3 {
                        next_board[i][j] = 1;
                    } else {
                        next_board[i][j] = 0;
                    }
                }
            }
        }
        
        *board = next_board;
    }
    
    fn count_live_neighbors(board: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        let m = board.len();
        let n = board[0].len();
        let directions = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1)
        ];
        
        let mut count = 0;
        
        for dir in directions {
            let new_row = (row as i32 + dir.0) as usize;
            let new_col = (col as i32 + dir.1) as usize;
            
            if new_row < m && new_col < n && board[new_row][new_col] == 1 {
                count += 1;
            }
        }
        
        count

    }
}
