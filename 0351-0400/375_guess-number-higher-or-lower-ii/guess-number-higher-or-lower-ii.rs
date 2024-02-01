
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 2]; n + 2];

        for len in 2..=n {
            for i in 1..=n-len+1 {
                let j = i + len - 1;
                dp[i][j] = std::i32::MAX;
                for x in i..=j {
                    dp[i][j] = dp[i][j].min(x as i32 + dp[i][x-1].max(dp[x+1][j]));
                }
            }
        }

        dp[1][n]
    }
}
