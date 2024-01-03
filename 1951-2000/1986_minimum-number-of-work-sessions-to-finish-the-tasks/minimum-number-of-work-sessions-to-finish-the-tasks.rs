
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let max_mask = 1 << n;
        let mut dp = vec![std::i32::MAX; max_mask];
        
        for mask in 1..max_mask {
            let mut remaining = mask;
            let mut time = 0;
            let mut count = 0;
            for i in 0..n {
                if (remaining & 1) == 1 {
                    time += tasks[i];
                    count += 1;
                }
                remaining >>= 1;
            }
            if time <= session_time {
                dp[mask] = 1;
            } else {
                for sub in 1..mask {
                    if (sub & mask) == sub {
                        dp[mask] = dp[mask].min(dp[sub] + dp[mask - sub]);
                    }
                }
            }
        }
        
        return dp[max_mask - 1];
    }
}
