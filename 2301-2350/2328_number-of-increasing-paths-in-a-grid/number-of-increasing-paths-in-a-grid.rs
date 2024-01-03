
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let modulo = 1_000_000_007;
        
        let mut dp = vec![vec![0; n]; m];
        let mut count = 0;
        
        for i in 0..m {
            for j in 0..n {
                count += Self::dfs(i, j, &grid, &mut dp, m, n, &modulo);
                count %= modulo;
            }
        }
        
        count

    }
    
    fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, m: usize, n: usize, modulo: &i32) -> i32 {
        if dp[i][j] != 0 {
            return dp[i][j];
        }
        
        let directions = vec![-1, 0, 1, 0, -1];
        let mut count = 1;
        
        for k in 0..4 {
            let x = i as i32 + directions[k];
            let y = j as i32 + directions[k + 1];
            
            if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && grid[x as usize][y as usize] > grid[i][j] {
                count += Self::dfs(x as usize, y as usize, grid, dp, m, n, modulo);
                count %= *modulo;
            }
        }
        
        dp[i][j] = count;
        count

    }
}
