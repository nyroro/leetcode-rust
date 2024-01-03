
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mod_num = 1000000007;
        let m = group.len();
        let mut dp = vec![vec![0; n as usize + 1]; min_profit as usize + 1];
        dp[0][0] = 1;

        for i in 0..m {
            let g = group[i] as usize;
            let p = profit[i] as usize;
            for j in (0..=min_profit as usize).rev() {
                for k in (g..=n as usize).rev() {
                    dp[j][k] = (dp[j][k] + dp[j.saturating_sub(p)][k.saturating_sub(g)]) % mod_num;
                }
            }
        }

        let mut result = 0;
        for i in 0..=n as usize {
            result = (result + dp[min_profit as usize][i]) % mod_num;
        }

        result

    }
}
