
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![0; n]; n];
        let mut max_leaf = vec![vec![0; n]; n];

        for i in 0..n {
            let mut cur_max = 0;
            for j in i..n {
                cur_max = cur_max.max(arr[j]);
                max_leaf[i][j] = cur_max;
            }
        }

        for len in 2..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                dp[i][j] = i32::MAX;
                for k in i..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k+1][j] + max_leaf[i][k] * max_leaf[k+1][j]);
                }
            }
        }

        dp[0][n-1]
    }
}
