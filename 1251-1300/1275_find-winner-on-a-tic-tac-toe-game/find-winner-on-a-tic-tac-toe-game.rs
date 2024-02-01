
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        // 创建一个3x3的棋盘数组，用于表示游戏状态

        let mut board = [[' '; 3]; 3];
        
        // 实现一个函数来检查游戏是否结束以及谁是赢家

        fn check_winner(board: &[[char; 3]; 3]) -> Option<char> {
            // 检查行、列和对角线是否有相同的棋子

            for i in 0..3 {
                if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
                    return Some(board[i][0]);
                }
                if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                    return Some(board[0][i]);
                }
            }
            if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
                return Some(board[0][0]);
            }
            if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
                return Some(board[0][2]);
            }
            // 检查是否还有空位

            for i in 0..3 {
                for j in 0..3 {
                    if board[i][j] == ' ' {
                        return None; // 游戏未结束

                    }
                }
            }
            Some('D') // 游戏结束，平局

        }
        
        // 根据玩家的移动更新棋盘状态

        for (i, mv) in moves.iter().enumerate() {
            let player = if i % 2 == 0 { 'A' } else { 'B' };
            let row = mv[0] as usize;
            let col = mv[1] as usize;
            board[row][col] = player;
        }
        
        // 根据游戏状态返回相应的结果

        match check_winner(&board) {
            Some('A') => "A".to_string(),
            Some('B') => "B".to_string(),
            Some('D') => "Draw".to_string(),
            None => "Pending".to_string(),
        }
    }
}
