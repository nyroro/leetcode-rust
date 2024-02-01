
impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![-1; (target + 1) as usize];
        dp[0] = 0;
        for &num in &nums {
            for i in (num..=target).rev() {
                if dp[(i - num) as usize] != -1 {
                    dp[i as usize] = dp[i as usize].max(dp[(i - num) as usize] + 1);
                }
            }
        }
        dp[target as usize]
    }
}
