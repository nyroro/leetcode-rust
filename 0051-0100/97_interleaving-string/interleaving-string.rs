
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let n = s1.len();
        let m = s2.len();
        let k = s3.len();
        
        if n + m != k {
            return false;
        }
        
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();
        
        let mut dp = vec![vec![false; m+1]; n+1];
        
        dp[0][0] = true;
        
        for i in 0..=n {
            for j in 0..=m {
                if i > 0 && s1[i-1] == s3[i+j-1] && dp[i-1][j] {
                    dp[i][j] = true;
                }
                if j > 0 && s2[j-1] == s3[i+j-1] && dp[i][j-1] {
                    dp[i][j] = true;
                }
            }
        }
        
        dp[n][m]
    }
}
