
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = vec![vec![0; 5]; (n+1) as usize];
        
        for j in 0..5 {
            dp[1][j] = 1;
        }
        
        for i in 2..=n as usize {
            for j in 0..5 {
                for k in j..5 {
                    dp[i][j] += dp[i-1][k];
                }
            }
        }
        
        let mut count = 0;
        for j in 0..5 {
            count += dp[n as usize][j];
        }
        
        count

    }
}
