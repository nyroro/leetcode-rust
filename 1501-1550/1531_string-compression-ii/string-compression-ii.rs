
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![101; n + 1]; n + 1];
        dp[0][0] = 0;
        
        for i in 1..=n {
            for j in 0..=k as usize {
                if j > 0 {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                let mut cnt = 0;
                let mut del = 0;
                for t in (1..=i).rev() {
                    if s[t - 1] == s[i - 1] {
                        cnt += 1;
                    } else {
                        del += 1;
                    }
                    if j as i32 - del as i32 < 0 {
                        break;
                    }
                    dp[i][j] = std::cmp::min(dp[i][j], dp[t - 1][j - del] + 1 + if cnt >= 100 { 3 } else if cnt >= 10 { 2 } else if cnt >= 2 { 1 } else { 0 });
                }
            }
        }
        
        dp[n][k as usize].min(n as i32)
    }
}
