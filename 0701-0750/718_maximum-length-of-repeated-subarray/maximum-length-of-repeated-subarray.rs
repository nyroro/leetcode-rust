
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 初始化一个二维数组dp，用于存储子数组的长度

        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut max_length = 0;
        
        // 遍历nums1和nums2，计算最长子数组的长度

        for i in (0..nums1.len()).rev() {
            for j in (0..nums2.len()).rev() {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                    max_length = max_length.max(dp[i][j]);
                }
            }
        }
        
        max_length

    }
}
