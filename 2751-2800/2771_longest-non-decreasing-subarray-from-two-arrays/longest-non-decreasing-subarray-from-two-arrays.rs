
impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![vec![1; 2]; n];
        let mut result = 1;

        for i in 0..n {
            if i > 0 {
                if nums1[i] >= nums1[i - 1] {
                    dp[i][0] = dp[i - 1][0] + 1;
                }
                if nums2[i] >= nums1[i - 1] {
                    dp[i][1] = dp[i - 1][0] + 1;
                }
                if nums2[i] >= nums2[i - 1] {
                    dp[i][1] = dp[i][1].max(dp[i - 1][1] + 1);
                }
                if nums1[i] >= nums2[i - 1] {
                    dp[i][0] = dp[i][0].max(dp[i - 1][1] + 1);
                }
            }
            result = result.max(dp[i][0]).max(dp[i][1]);
        }

        result

    }
}
