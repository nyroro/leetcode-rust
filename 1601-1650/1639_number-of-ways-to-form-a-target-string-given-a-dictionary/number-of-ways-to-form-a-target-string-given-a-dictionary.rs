
use std::collections::HashMap;



impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (m, n) = (words[0].len(), target.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut count: HashMap<(usize, char), i64> = HashMap::new();

        for word in &words {
            for (i, c) in word.chars().enumerate() {
                *count.entry((i, c)).or_insert(0) += 1;
            }
        }

        for i in 0..=m {
            dp[i][0] = 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                let char_count = count.get(&(i - 1, target.chars().nth(j - 1).unwrap())).unwrap_or(&0);
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1] * char_count % MOD) % MOD;
            }
        }

        dp[m][n] as i32

    }
}
