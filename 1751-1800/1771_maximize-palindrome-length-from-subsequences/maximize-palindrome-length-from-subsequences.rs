
impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let new_word = format!("{}{}", word1, word2);
        let n = new_word.len();
        let new_word_bytes = new_word.as_bytes();
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            dp[i][i] = 1;
        }

        let mut res = 0;
        for len in 2..=n {
            for i in 0..n - len + 1 {
                let j = i + len - 1;
                if new_word_bytes[i] == new_word_bytes[j] {
                    dp[i][j] = if len == 2 { 2 } else { dp[i + 1][j - 1] + 2 };
                    if i < word1.len() && j >= word1.len() {
                        res = res.max(dp[i][j]);
                    }
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }

        res

    }
}
