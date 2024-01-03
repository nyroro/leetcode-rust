
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let m = key.len();
        let ring_chars: Vec<char> = ring.chars().collect();
        let key_chars: Vec<char> = key.chars().collect();
        let mut dp = vec![vec![0; m + 1]; n];
        
        for j in (0..m).rev() {
            for i in 0..n {
                dp[i][j] = std::i32::MAX;
                for k in 0..n {
                    if ring_chars[k] == key_chars[j] {
                        let diff = (i as i32 - k as i32).abs();
                        let steps = std::cmp::min(diff, n as i32 - diff);
                        dp[i][j] = std::cmp::min(dp[i][j], steps + dp[k][j + 1]);
                    }
                }
            }
        }
        
        dp[0][0] + m as i32

    }
}
