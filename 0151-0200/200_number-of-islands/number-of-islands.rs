
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' && !visited[i][j] {
                    Self::dfs(&grid, &mut visited, i, j);
                    count += 1;
                }
            }
        }
        
        count

    }
    
    fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) {
        if row >= grid.len() || col >= grid[0].len() || grid[row][col] == '0' || visited[row][col] {
            return;
        }
        
        visited[row][col] = true;
        
        Self::dfs(grid, visited, row + 1, col);
        Self::dfs(grid, visited, row - 1, col);
        Self::dfs(grid, visited, row, col + 1);
        Self::dfs(grid, visited, row, col - 1);
    }
}
