
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dp = vec![vec![vec![-1; cols]; cols]; rows];
        Solution::dfs(0, 0, cols - 1, &grid, &mut dp)
    }

    fn dfs(x: usize, y1: usize, y2: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        if x == rows {
            return 0;
        }
        if dp[x][y1][y2] != -1 {
            return dp[x][y1][y2];
        }
        let mut cherries = grid[x][y1];
        if y1 != y2 {
            cherries += grid[x][y2];
        }
        let mut result = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let ny1 = (y1 as i32 + i) as usize;
                let ny2 = (y2 as i32 + j) as usize;
                if ny1 < cols && ny2 < cols {
                    result = result.max(Solution::dfs(x + 1, ny1, ny2, grid, dp));
                }
            }
        }
        dp[x][y1][y2] = cherries + result;
        dp[x][y1][y2]
    }
}
