
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 2]; 32];
        dp[0][0] = 1;
        dp[0][1] = 1;
        
        for i in 1..32 {
            dp[i][0] = dp[i-1][0] + dp[i-1][1];
            dp[i][1] = dp[i-1][0];
        }
        
        let mut pre_bit = 0;
        let mut res = 0;
        
        for i in (0..32).rev() {
            let bit = (n >> i) & 1;
            if bit == 1 {
                res += dp[i][0];
            }
            if pre_bit == 1 && bit == 1 {
                break;
            }
            pre_bit = bit;
            if i == 0 {
                res += 1;
            }
        }
        
        res

    }
}
