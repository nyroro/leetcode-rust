
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for (i, &range) in ranges.iter().enumerate() {
            let left = (i as i32 - range).max(0) as usize;
            let right = (i as i32 + range).min(n as i32) as usize;
            for j in left..=right {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }

        let mut min_taps = std::i32::MIN;
        for i in 0..=n {
            if dp[i] == std::i32::MAX {
                return -1;
            }
            min_taps = min_taps.max(dp[i]);
        }

        min_taps

    }
}
