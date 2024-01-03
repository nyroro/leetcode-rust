
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        
        let mut is_palindrome = vec![vec![false; n]; n];
        let mut dp = vec![0; n];
        
        for i in 0..n {
            is_palindrome[i][i] = true;
        }
        
        for len in 2..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                if s[i] == s[j] && (len == 2 || is_palindrome[i+1][j-1]) {
                    is_palindrome[i][j] = true;
                }
            }
        }
        
        for j in 0..n {
            if is_palindrome[0][j] {
                dp[j] = 0;
            } else {
                dp[j] = j as i32;
                for i in 0..j {
                    if is_palindrome[i+1][j] {
                        dp[j] = dp[j].min(dp[i] + 1);
                    }
                }
            }
        }
        
        dp[n-1]
    }
}
