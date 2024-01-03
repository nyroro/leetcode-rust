
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        
        for i in 2..=n {
            for j in 1..i {
                dp[i as usize] = dp[i as usize].max(j * (i - j));
                dp[i as usize] = dp[i as usize].max(j * dp[(i - j) as usize]);
            }
        }
        
        dp[n as usize]
    }
}
