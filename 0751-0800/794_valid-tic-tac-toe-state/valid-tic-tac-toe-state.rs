
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut x_count = 0;
        let mut o_count = 0;
        
        // 统计X和O的数量

        for row in &board {
            for c in row.chars() {
                if c == 'X' {
                    x_count += 1;
                } else if c == 'O' {
                    o_count += 1;
                }
            }
        }
        
        // 判断X和O的数量是否合法

        if x_count != o_count && x_count != o_count + 1 {
            return false;
        }
        
        // 判断是否有玩家已经获胜

        if Self::check_win(&board, 'X') && x_count != o_count + 1 {
            return false;
        }
        if Self::check_win(&board, 'O') && x_count != o_count {
            return false;
        }
        
        true

    }
    
    // 检查指定玩家是否获胜

    fn check_win(board: &Vec<String>, player: char) -> bool {
        // 检查行

        for row in board {
            if row.chars().all(|c| c == player) {
                return true;
            }
        }
        
        // 检查列

        for i in 0..3 {
            if board[0].chars().nth(i) == Some(player) &&
               board[1].chars().nth(i) == Some(player) &&
               board[2].chars().nth(i) == Some(player) {
                return true;
            }
        }
        
        // 检查对角线

        if (board[0].chars().nth(0) == Some(player) &&
            board[1].chars().nth(1) == Some(player) &&
            board[2].chars().nth(2) == Some(player)) ||
           (board[0].chars().nth(2) == Some(player) &&
            board[1].chars().nth(1) == Some(player) &&
            board[2].chars().nth(0) == Some(player)) {
            return true;
        }
        
        false

    }
}
