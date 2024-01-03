
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let n = nums.len();
        let mut dp = vec![vec![false; (target + 1) as usize]; (n + 1) as usize];
        for i in 0..=n {
            dp[i][0] = true;
        }
        for i in 1..=n {
            for j in 1..=target {
                if j < nums[i - 1] {
                    dp[i][j as usize] = dp[i - 1][j as usize];
                } else {
                    dp[i][j as usize] = dp[i - 1][j as usize] || dp[i - 1][(j - nums[i - 1]) as usize];
                }
            }
        }
        dp[n][target as usize]
    }
}
