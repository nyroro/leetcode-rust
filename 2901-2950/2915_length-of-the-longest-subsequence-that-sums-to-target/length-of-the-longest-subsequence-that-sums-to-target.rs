
impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;
        for num in nums {
            for i in (num..=target).rev() {
                if i - num >= 0 {
                    dp[i as usize] = dp[i as usize] + dp[(i - num) as usize];
                }
            }
        }
        if dp[target as usize] == 0 {
            return -1;
        }
        dp[target as usize]
    }
}
