
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![s.len() as i32 + 1; s.len() + 1];
        dp[0] = 0;

        for i in 1..=s.len() {
            for j in 0..i {
                if dictionary.contains(&s[j..i].to_string()) {
                    dp[i] = dp[i].min(dp[j]);
                } else {
                    dp[i] = dp[i].min(dp[j] + (i - j) as i32);
                }
            }
        }

        dp[s.len()]
    }
}
