
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 1001]; n];
        let mut max_len = 0;

        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] - nums[j] + 500;
                dp[i][diff] = dp[j][diff] + 1;
                max_len = max_len.max(dp[i][diff]);
            }
        }

        max_len + 1

    }
}
