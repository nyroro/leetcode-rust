
impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        
        let mut is_palindrome = vec![vec![false; n]; n];
        for len in 1..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                if len == 1 {
                    is_palindrome[i][j] = true;
                } else if len == 2 {
                    is_palindrome[i][j] = s[i] == s[j];
                } else {
                    is_palindrome[i][j] = s[i] == s[j] && is_palindrome[i+1][j-1];
                }
            }
        }
        
        let mut dp = vec![0; n];
        for i in 0..n {
            for j in 0..=i {
                if is_palindrome[j][i] && i - j + 1 >= k {
                    if j > 0 {
                        dp[i] = dp[i].max(dp[j-1] + 1);
                    } else {
                        dp[i] = dp[i].max(1);
                    }
                }
            }
            if i > 0 {
                dp[i] = dp[i].max(dp[i-1]);
            }
        }
        
        dp[n-1]
    }
}
