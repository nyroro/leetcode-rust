
impl Solution {
    fn is_valid(row: i32, col: i32) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8

    }

    fn dfs(board: &Vec<Vec<char>>, color: char, row: i32, col: i32, dir: (i32, i32)) -> bool {
        if board[row as usize][col as usize] == color {
            return true;
        }
        if board[row as usize][col as usize] == '.' {
            return false;
        }
        let (dx, dy) = dir;
        let new_row = row + dx;
        let new_col = col + dy;
        if Solution::is_valid(new_row, new_col) && Solution::dfs(board, color, new_row, new_col, dir) {
            return true;
        }
        false

    }

    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let directions = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        if board[r_move as usize][c_move as usize] == 'B' || board[r_move as usize][c_move as usize] == 'W' {
            return false;
        }

        for dir in &directions {
            let (dx, dy) = dir;
            let new_row = r_move + dx;
            let new_col = c_move + dy;
            if Solution::is_valid(new_row, new_col) && board[new_row as usize][new_col as usize] != color

                && Solution::dfs(&board, color, new_row, new_col, *dir)
            {
                return true;
            }
        }
        false

    }
}
