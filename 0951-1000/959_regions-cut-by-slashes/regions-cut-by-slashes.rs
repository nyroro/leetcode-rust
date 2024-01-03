
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut regions = 0;
        let mut visited = vec![vec![false; n * 3]; n * 3];
        
        for i in 0..n {
            let chars: Vec<char> = grid[i].chars().collect();
            for j in 0..n {
                if chars[j] == '/' {
                    visited[i * 3][j * 3 + 2] = true;
                    visited[i * 3 + 1][j * 3 + 1] = true;
                    visited[i * 3 + 2][j * 3] = true;
                } else if chars[j] == '\\' {
                    visited[i * 3][j * 3] = true;
                    visited[i * 3 + 1][j * 3 + 1] = true;
                    visited[i * 3 + 2][j * 3 + 2] = true;
                }
            }
        }
        
        for i in 0..(n * 3) {
            for j in 0..(n * 3) {
                if !visited[i][j] {
                    Self::dfs(&mut visited, i, j);
                    regions += 1;
                }
            }
        }
        
        regions

    }
    
    fn dfs(visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if i >= visited.len() || j >= visited[0].len() || visited[i][j] {
            return;
        }
        
        visited[i][j] = true;
        Self::dfs(visited, i + 1, j);
        Self::dfs(visited, i - 1, j);
        Self::dfs(visited, i, j + 1);
        Self::dfs(visited, i, j - 1);
    }
}
