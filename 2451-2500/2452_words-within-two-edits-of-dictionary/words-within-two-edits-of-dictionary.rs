
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        // 定义一个函数来计算两个字符串之间的Levenshtein距离

        fn levenshtein_distance(s: &str, t: &str) -> usize {
            let n = s.chars().count();
            let m = t.chars().count();
            let mut dp = vec![vec![0; m + 1]; n + 1];
            for i in 0..=n {
                dp[i][0] = i;
            }
            for j in 0..=m {
                dp[0][j] = j;
            }
            for (i, s_char) in s.chars().enumerate() {
                for (j, t_char) in t.chars().enumerate() {
                    if s_char == t_char {
                        dp[i + 1][j + 1] = dp[i][j];
                    } else {
                        dp[i + 1][j + 1] = dp[i][j].min(dp[i][j + 1].min(dp[i + 1][j])) + 1;
                    }
                }
            }
            dp[n][m]
        }

        let mut result = Vec::new();
        for query in queries {
            let mut matched = false;
            for word in &dictionary {
                if levenshtein_distance(&query, word) <= 2 {
                    matched = true;
                    break;
                }
            }
            if matched {
                result.push(query);
            }
        }
        result

    }
}
