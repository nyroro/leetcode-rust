
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![-1; n]; n]; n];
        let result = Solution::dfs(0, 0, 0, &grid, &mut dp);
        if result < 0 {
            0

        } else {
            result

        }
    }

    fn dfs(x1: usize, y1: usize, x2: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let y2 = x1 + y1 - x2;
        let n = grid.len();
        if x1 >= n || y1 >= n || x2 >= n || y2 >= n || grid[x1][y1] == -1 || grid[x2][y2] == -1 {
            return -9999;
        }
        if x1 == n - 1 && y1 == n - 1 {
            return grid[x1][y1];
        }
        if dp[x1][y1][x2] != -1 {
            return dp[x1][y1][x2];
        }
        let mut result = grid[x1][y1];
        if x1 != x2 {
            result += grid[x2][y2];
        }
        result += i32::max(
            i32::max(Solution::dfs(x1 + 1, y1, x2 + 1, grid, dp), Solution::dfs(x1, y1 + 1, x2, grid, dp)),
            i32::max(Solution::dfs(x1 + 1, y1, x2, grid, dp), Solution::dfs(x1, y1 + 1, x2 + 1, grid, dp)),
        );
        dp[x1][y1][x2] = result;
        result

    }
}
