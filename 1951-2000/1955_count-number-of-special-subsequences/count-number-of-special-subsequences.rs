
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let mut dp = vec![0, 0, 0]; // dp[0]表示以0结尾的特殊子序列数量，dp[1]表示以1结尾的特殊子序列数量，dp[2]表示以2结尾的特殊子序列数量

        for &num in nums.iter() {
            dp[num as usize] = ((dp[num as usize] * 2) % modulo + if num == 0 { 1 } else { dp[(num - 1) as usize] }) % modulo;
        }
        dp[2]
    }
}
