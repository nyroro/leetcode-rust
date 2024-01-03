
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        
        let mut dp = vec![0; (n + 1) as usize];
        dp[1] = 1;
        let mut max_num = 1;
        
        for i in 2..=n as usize {
            if i % 2 == 0 {
                dp[i] = dp[i / 2];
            } else {
                dp[i] = dp[i / 2] + dp[i / 2 + 1];
            }
            
            max_num = max_num.max(dp[i]);
        }
        
        max_num

    }
}
