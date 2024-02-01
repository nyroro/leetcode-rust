
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let modulo = 1000000007;
        let mut dp = vec![vec![vec![0; 16]; 7]; n as usize + 1];
        
        for j in 1..=6 {
            dp[1][j][1] = 1;
        }
        
        for i in 2..=n as usize {
            for j in 1..=6 {
                for k in 1..=roll_max[j-1] as usize {
                    if k == 1 {
                        for prev_j in 1..=6 {
                            if prev_j != j {
                                for prev_k in 1..=roll_max[prev_j-1] as usize {
                                    dp[i][j][k] += dp[i-1][prev_j][prev_k];
                                    dp[i][j][k] %= modulo;
                                }
                            }
                        }
                    } else {
                        dp[i][j][k] = dp[i-1][j][k-1];
                    }
                }
            }
        }
        
        let mut result = 0;
        for j in 1..=6 {
            for k in 1..=roll_max[j-1] as usize {
                result += dp[n as usize][j][k];
                result %= modulo;
            }
        }
        
        result as i32

    }
}
