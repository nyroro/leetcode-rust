
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        cuts.sort();
        let m = cuts.len();
        let mut dp = vec![vec![0; m + 2]; m + 2];
        let mut new_cuts = vec![0; m + 2];
        new_cuts[0] = 0;
        new_cuts[m + 1] = n;
        for i in 1..=m {
            new_cuts[i] = cuts[i - 1];
        }
        for len in 2..=m + 1 {
            for i in 0..=m + 1 - len {
                let j = i + len;
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + new_cuts[j] - new_cuts[i]);
                }
            }
        }
        dp[0][m + 1]
    }
}
