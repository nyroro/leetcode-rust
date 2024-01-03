
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let sum: usize = rods.iter().sum::<i32>() as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; sum * 2 + 1]; rods.len() + 1];
        dp[0][sum as usize] = 0;

        for i in 0..rods.len() {
            for j in 0..=sum * 2 {
                if dp[i][j] < 0 {
                    continue;
                }
                if j as i32 - rods[i] >= 0 {
                    dp[i + 1][(j as i32 - rods[i]) as usize] = dp[i + 1][(j as i32 - rods[i]) as usize].max(dp[i][j]);
                }
                dp[i + 1][(j as i32 + rods[i]) as usize] = dp[i + 1][(j as i32 + rods[i]) as usize].max(dp[i][j] + rods[i]);
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            }
        }

        dp[rods.len()][sum as usize]
    }
}
