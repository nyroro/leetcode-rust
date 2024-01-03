
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        let mut dp = vec![vec![false; sum as usize + 1]; n / 2 + 1];
        dp[0][0] = true;

        for &num in &nums {
            for i in (1..=n / 2).rev() {
                for j in num as usize..=sum as usize {
                    dp[i][j] |= dp[i - 1][j - num as usize];
                }
            }
        }

        for i in 1..=n / 2 {
            if sum * i as i32 % n as i32 == 0 && dp[i][sum as usize * i as usize / n] {
                return true;
            }
        }

        false

    }
}
