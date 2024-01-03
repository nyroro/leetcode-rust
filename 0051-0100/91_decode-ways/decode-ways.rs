
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        
        if n == 0 || s[0] == '0' {
            return 0;
        }
        
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        
        for i in 1..=n {
            if s[i - 1] != '0' {
                dp[i] += dp[i - 1];
            }
            
            if i > 1 && (s[i - 2] == '1' || (s[i - 2] == '2' && s[i - 1] <= '6')) {
                dp[i] += dp[i - 2];
            }
        }
        
        dp[n]
    }
}
