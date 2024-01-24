
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        if grid[0][0] == ')' || grid[n-1][m-1] == '(' {
            return false;
        }
        if (n + m - 1) % 2 != 0 {
            return false;
        }
        let v = (n + m) / 2 + 1;
        let mut dp = vec![vec![vec![false; v]; m]; n];
        dp[0][0][1] = true;

        for i in 0..n {
            for j in 0..m {
                for k in 0..v {
                    if i > 0 {
                        if grid[i][j] == '(' && k > 0 && dp[i-1][j][k-1] {
                            dp[i][j][k] = true;
                        }
                        if grid[i][j] == ')' && k < v - 1 && dp[i-1][j][k+1] {
                            dp[i][j][k] = true;
                        }
                    }
                    if j > 0 {
                        if grid[i][j] == '(' && k > 0 && dp[i][j-1][k-1] {
                            dp[i][j][k] = true;
                        }
                        if grid[i][j] == ')' && k < v - 1 && dp[i][j-1][k+1] {
                            dp[i][j][k] = true;
                        }
                    }
                }
            }
        }
        return dp[n-1][m-1][0];
    }
}
