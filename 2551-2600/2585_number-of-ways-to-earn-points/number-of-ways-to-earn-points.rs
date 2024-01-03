
impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let n = types.len();
        let m = 1000; // 题目中给出的最大target值

        let mut dp = vec![vec![0; m as usize + 1]; n + 1];
        dp[0][0] = 1;
        
        for i in 1..=n {
            let (count, marks) = (types[i - 1][0] as usize, types[i - 1][1] as usize);
            for j in 0..=m {
                dp[i][j] = dp[i - 1][j];
                for k in 1..=count {
                    if j >= k * marks {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - k * marks]) % 1000000007;
                    } else {
                        break;
                    }
                }
            }
        }
        
        dp[n][target as usize]
    }
}
