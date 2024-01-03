
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        for i in 2..=n {
            dp[i as usize] = i;
            for j in (2..i).rev() {
                if i % j == 0 {
                    dp[i as usize] = dp[j as usize] + i / j;
                    break;
                }
            }
        }
        dp[n as usize]
    }
}
