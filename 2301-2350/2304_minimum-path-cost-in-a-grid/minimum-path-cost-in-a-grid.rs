
impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![std::i32::MAX; n]; m];
        
        // 初始化第一行的成本

        for j in 0..n {
            dp[0][j] = grid[0][j];
        }
        
        // 计算从第二行到最后一行的最小路径成本

        for i in 1..m {
            for j in 0..n {
                for k in 0..n {
                    let cost = dp[i - 1][k] + move_cost[grid[i - 1][k] as usize][j];
                    dp[i][j] = dp[i][j].min(cost);
                }
                dp[i][j] += grid[i][j];
            }
        }
        
        // 返回最后一行中的最小值

        *dp.last().unwrap().iter().min().unwrap()
    }
}
