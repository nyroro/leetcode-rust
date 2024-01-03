
impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = 1;
            dp[i][i] = 1;
        }
        for i in 2..=n {
            for j in 1..i {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD;
            }
        }
        fn dfs(nums: &[i32], dp: &Vec<Vec<i64>>) -> i64 {
            let n = nums.len();
            if n <= 2 {
                return 1;
            }
            let (mut left, mut right) = (Vec::new(), Vec::new());
            for &num in nums.iter().skip(1) {
                if num < nums[0] {
                    left.push(num);
                } else {
                    right.push(num);
                }
            }
            let left_count = left.len();
            let right_count = right.len();
            let left_ways = dfs(&left, dp);
            let right_ways = dfs(&right, dp);
            let total_ways = dp[left_count + right_count][left_count];
            (left_ways * right_ways % MOD) * total_ways % MOD

        }
        dfs(&nums, &dp) as i32 - 1

    }
}
