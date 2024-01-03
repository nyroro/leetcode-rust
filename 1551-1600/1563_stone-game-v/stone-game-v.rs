
impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stone_value[i];
        }
        
        let mut dp = vec![vec![0; n]; n];
        
        fn dfs(i: usize, j: usize, prefix_sum: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if i == j {
                return 0;
            }
            if dp[i][j] > 0 {
                return dp[i][j];
            }
            let mut res = 0;
            for k in i..j {
                let left_sum = prefix_sum[k + 1] - prefix_sum[i];
                let right_sum = prefix_sum[j + 1] - prefix_sum[k + 1];
                if left_sum <= right_sum {
                    res = res.max(left_sum + dfs(i, k, prefix_sum, dp));
                }
                if left_sum >= right_sum {
                    res = res.max(right_sum + dfs(k + 1, j, prefix_sum, dp));
                }
            }
            dp[i][j] = res;
            return res;
        }
        
        dfs(0, n - 1, &prefix_sum, &mut dp)
    }
}
