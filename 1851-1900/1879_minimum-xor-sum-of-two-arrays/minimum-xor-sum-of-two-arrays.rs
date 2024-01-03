
impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![vec![std::i32::MAX; 1 << n]; n + 1];
        dp[0][0] = 0;
        
        for i in 1..=n {
            for mask in 0..1 << n {
                for j in 0..n {
                    if mask & (1 << j) == 0 {
                        dp[i][mask | (1 << j)] = dp[i][mask | (1 << j)].min(dp[i - 1][mask] + (nums1[i - 1] ^ nums2[j]));
                    }
                }
            }
        }
        
        dp[n][(1 << n) - 1]
    }
}
