
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();
        
        for i in 0..rows {
            for j in 0..cols {
                if Self::dfs(&board, &word_chars, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }
        
        false

    }
    
    fn dfs(board: &Vec<Vec<char>>, word: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
        if i < 0 || i as usize >= board.len() || j < 0 || j as usize >= board[0].len() || board[i as usize][j as usize] != word[k] {
            return false;
        }
        
        if k == word.len() - 1 {
            return true;
        }
        
        let temp = board[i as usize][j as usize];
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut result = false;
        
        board[i as usize][j as usize] = ' '; // 标记当前单元格已访问
        
        for (dx, dy) in directions {
            let x = i + dx;
            let y = j + dy;
            if Self::dfs(board, word, x, y, k + 1) {
                result = true;
                break;
            }
        }
        
        board[i as usize][j as usize] = temp; // 恢复当前单元格的值

        result

    }
}
