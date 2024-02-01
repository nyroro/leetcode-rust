
use std::collections::HashMap;

impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        let mut dp: Vec<i64> = vec![0; s.len() + 1];
        let mut sum: i64 = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 1..=s.len() {
            dp[i] = dp[i - 1] + i as i64 - last_seen.get(&chars[i - 1]).unwrap_or(&0) as i64;
            last_seen.insert(chars[i - 1], i);
            sum += dp[i];
        }

        sum

    }
}
