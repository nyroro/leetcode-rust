
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![(0, 1); n];
        let inf = 1_000_000_000;

        for i in (0..n - 1).rev() {
            dp[i] = (inf, inf);

            if nums1[i] <= nums1[n - 1] && nums2[i] <= nums2[n - 1] {
                dp[i].0 = dp[i].0.min(dp[i + 1].0);
                dp[i].1 = dp[i].1.min(dp[i + 1].1 + 1);
            }

            if nums1[i] <= nums2[n - 1] && nums2[i] <= nums1[n - 1] {
                dp[i].0 = dp[i].0.min(dp[i + 1].0 + 1);
                dp[i].1 = dp[i].1.min(dp[i + 1].1);
            }
        }

        let best = dp[0].0.min(dp[0].1);

        if best > n as i32 {
            return -1;
        }

        best

    }
}
