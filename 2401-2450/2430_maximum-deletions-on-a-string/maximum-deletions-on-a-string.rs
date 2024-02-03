


impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let n = s.len();
        if s.chars().all(|c| c == s.chars().next().unwrap()) {
            return n as i32;
        }
        let mut p = vec![0; n + 1];
        let mut dp = vec![1; n];
        let s_chars: Vec<char> = s.chars().collect();
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                p[j] = if s_chars[i] == s_chars[j] { p[j + 1] + 1 } else { 0 };
                if p[j] >= j - i {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp[0]
    }
}
