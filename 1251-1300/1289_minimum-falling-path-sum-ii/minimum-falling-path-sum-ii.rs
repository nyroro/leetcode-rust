
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![0; n]; n];
        
        // 初始化dp数组的第一行

        for j in 0..n {
            dp[0][j] = grid[0][j];
        }
        
        // 填充dp数组

        for i in 1..n {
            for j in 0..n {
                dp[i][j] = grid[i][j] + Self::get_min(&dp[i-1], j);
            }
        }
        
        // 返回dp数组最后一行的最小值

        *dp[n-1].iter().min().unwrap()
    }
    
    // 辅助函数，用于获取数组中除了指定索引的最小值

    fn get_min(arr: &[i32], exclude_index: usize) -> i32 {
        let mut min_val = i32::MAX;
        for (i, &num) in arr.iter().enumerate() {
            if i != exclude_index && num < min_val {
                min_val = num;
            }
        }
        min_val

    }
}
