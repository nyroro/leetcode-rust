
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![vec![0; m + 1]; m + 1];
        
        fn dfs(i: usize, j: usize, k: usize, nums: &Vec<i32>, multipliers: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if k == multipliers.len() {
                return 0;
            }
            if dp[i][k] != 0 {
                return dp[i][k];
            }
            let left = multipliers[k] * nums[i] + dfs(i + 1, j, k + 1, nums, multipliers, dp);
            let right = multipliers[k] * nums[j] + dfs(i, j - 1, k + 1, nums, multipliers, dp);
            dp[i][k] = left.max(right);
            dp[i][k]
        }
        
        dfs(0, n - 1, 0, &nums, &multipliers, &mut dp)
    }
}
