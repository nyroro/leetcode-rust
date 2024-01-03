
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let len1 = str1.len();
        let len2 = str2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            for j in 0..=len2 {
                if i == 0 {
                    dp[i][j] = j;
                } else if j == 0 {
                    dp[i][j] = i;
                } else if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + 1;
                }
            }
        }

        let mut res = String::new();
        let mut i = len1;
        let mut j = len2;

        while i > 0 && j > 0 {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                res.push(str1.chars().nth(i - 1).unwrap());
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] < dp[i][j - 1] {
                res.push(str1.chars().nth(i - 1).unwrap());
                i -= 1;
            } else {
                res.push(str2.chars().nth(j - 1).unwrap());
                j -= 1;
            }
        }

        while i > 0 {
            res.push(str1.chars().nth(i - 1).unwrap());
            i -= 1;
        }

        while j > 0 {
            res.push(str2.chars().nth(j - 1).unwrap());
            j -= 1;
        }

        res.chars().rev().collect()
    }
}
