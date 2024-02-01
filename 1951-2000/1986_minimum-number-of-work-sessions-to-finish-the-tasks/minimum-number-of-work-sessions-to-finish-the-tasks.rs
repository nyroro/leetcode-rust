
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let mut dp = vec![std::i32::MAX; 1 << n];
        let mut ok = vec![false; 1 << n];
        dp[0] = 0;
        
        for i in 1..(1 << n) {
            let mut t = 0;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    t += tasks[j];
                }
            }
            ok[i] = t <= session_time;
        }
        
        for i in 1..(1 << n) {
            for j in (1..=i).rev() {
                let mut ms = j;
                while ms > 0 {
                    if ok[ms as usize] {
                        dp[i] = std::cmp::min(dp[i], dp[i ^ ms as usize] + 1);
                    }
                    ms = (ms - 1) & j;
                }
            }
        }
        
        return dp[(1 << n) - 1];
    }
}
