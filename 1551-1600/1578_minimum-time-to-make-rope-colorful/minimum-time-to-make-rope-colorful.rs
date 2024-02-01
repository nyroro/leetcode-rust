


impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut s = needed_time[0];
        let n = needed_time.len();
        let mut dp = vec![0; n];
        let mut now = 0;
        let chars = colors.chars();
        for i in 1..n {
            if chars.nth(i).unwrap() == chars.nth(now).unwrap() {
                let pre = if now == 0 { 0 } else { dp[now - 1] };
                let val = s;
                dp[i] = std::cmp::min(pre + val, dp[i - 1] + needed_time[i]);
                s += needed_time[i];
            } else {
                dp[i] = dp[i - 1];
                s = needed_time[i];
                now = i;
            }
        }

        dp[n - 1]
    }
}
