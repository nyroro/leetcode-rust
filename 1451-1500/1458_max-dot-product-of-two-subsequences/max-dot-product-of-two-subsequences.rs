
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n]; m];

        dp[0][0] = nums1[0] * nums2[0];

        for i in 1..m {
            dp[i][0] = nums1[i] * nums2[0];
            dp[i][0] = dp[i][0].max(dp[i-1][0]);
        }

        for j in 1..n {
            dp[0][j] = nums1[0] * nums2[j];
            dp[0][j] = dp[0][j].max(dp[0][j-1]);
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = nums1[i] * nums2[j];
                dp[i][j] = dp[i][j].max(dp[i-1][j-1] + nums1[i] * nums2[j]);
                dp[i][j] = dp[i][j].max(dp[i][j-1]);
                dp[i][j] = dp[i][j].max(dp[i-1][j]);
            }
        }

        dp[m-1][n-1]
    }
}
