
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut dp = vec![vec![false; n]; n];
        let mut start = 0;
        let mut max_len = 1;

        // 单个字符都是回文串

        for i in 0..n {
            dp[i][i] = true;
        }

        // 判断长度为2的子串是否是回文串

        for i in 0..n-1 {
            if chars[i] == chars[i+1] {
                dp[i][i+1] = true;
                start = i;
                max_len = 2;
            }
        }

        // 判断长度大于2的子串是否是回文串

        for len in 3..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                if chars[i] == chars[j] && dp[i+1][j-1] {
                    dp[i][j] = true;
                    start = i;
                    max_len = len;
                }
            }
        }

        s[start..start+max_len].to_string()
    }
}
