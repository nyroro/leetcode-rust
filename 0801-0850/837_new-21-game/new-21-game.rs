
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let mut dp = vec![0.0; (n + 1) as usize];
        dp[0] = 1.0;
        let mut s = if k > 0 { 1.0 } else { 0.0 };
        for i in 1..=n {
            dp[i as usize] = s / max_pts as f64;
            if i < k {
                s += dp[i as usize];
            }
            if i - max_pts >= 0 && i - max_pts < k {
                s -= dp[(i - max_pts) as usize];
            }
        }
        let mut ans = 0.0;
        for i in k..=n {
            ans += dp[i as usize];
        }
        ans

    }
}
