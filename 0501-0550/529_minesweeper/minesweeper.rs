
impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let (row, col) = (click[0] as usize, click[1] as usize);
        let m = board.len();
        let n = board[0].len();
        
        if board[row][col] == 'M' {
            board[row][col] = 'X';
            return board;
        }
        
        let mut count = 0;
        let directions = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1)
        ];
        
        for &(dx, dy) in &directions {
            let (new_row, new_col) = (row as i32 + dx, col as i32 + dy);
            if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                if board[new_row][new_col] == 'M' {
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            board[row][col] = (count as u8 + b'0') as char;
        } else {
            board[row][col] = 'B';
            for &(dx, dy) in &directions {
                let (new_row, new_col) = (row as i32 + dx, col as i32 + dy);
                if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                    let (new_row, new_col) = (new_row as usize, new_col as usize);
                    if board[new_row][new_col] == 'E' {
                        let new_click = vec![new_row as i32, new_col as i32];
                        board = Solution::update_board(board, new_click);
                    }
                }
            }
        }
        
        board

    }
}
