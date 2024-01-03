
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();
        
        for i in 0..rows {
            for j in 0..cols {
                let mut visited = vec![vec![false; cols]; rows];
                if Self::dfs(&board, &word_chars, &mut visited, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }
        
        false

    }
    
    fn dfs(board: &Vec<Vec<char>>, word: &Vec<char>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32, k: usize) -> bool {
        if i < 0 || i as usize >= board.len() || j < 0 || j as usize >= board[0].len() || visited[i as usize][j as usize] || board[i as usize][j as usize] != word[k] {
            return false;
        }
        
        if k == word.len() - 1 {
            return true;
        }
        
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        visited[i as usize][j as usize] = true;
        
        for (dx, dy) in directions {
            let x = i + dx;
            let y = j + dy;
            if Self::dfs(board, word, visited, x, y, k + 1) {
                return true;
            }
        }
        
        visited[i as usize][j as usize] = false;
        false

    }
}
