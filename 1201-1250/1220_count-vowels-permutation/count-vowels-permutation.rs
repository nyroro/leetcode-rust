impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let modulo = 1000000007;
        let mut dp = vec![vec![0; 5]; n as usize];
        
        for i in 0..5 {
            dp[0][i] = 1;
        }
        
        for i in 1..n as usize {
            dp[i][0] = ((dp[i-1][1] + dp[i-1][2])%modulo + dp[i-1][4]) % modulo;
            dp[i][1] = (dp[i-1][0] + dp[i-1][2]) % modulo;
            dp[i][2] = (dp[i-1][1] + dp[i-1][3]) % modulo;
            dp[i][3] = dp[i-1][2];
            dp[i][4] = (dp[i-1][2] + dp[i-1][3]) % modulo;
        }
        
        let mut result = 0;
        for i in 0..5 {
            result = (result + dp[n as usize - 1][i]) % modulo;
        }
        
        result as i32

    }
}
