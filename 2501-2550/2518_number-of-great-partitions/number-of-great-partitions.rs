


impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let sum1: i64 = nums.iter().map(|&i| i as i64).sum();
        if sum1 < 2 * k as i64 || nums.len() <= 1 {
            return 0;
        }
        let n = nums.len();
        let mod_val = 1_000_000_007;
        let mut dp = vec![vec![-1; k as usize]; n + 1];
        for i in 0..=n {
            for j in 0..k as usize {
                if i == 0 {
                    dp[i][j] = 0;
                }
                if j == 0 {
                    dp[i][j] = 1;
                }
                if i > 0 && j > 0 {
                    if j as i32 >= nums[i - 1] {
                        dp[i][j] = (dp[i - 1][j] as i64 + dp[i - 1][j - nums[i - 1] as usize] as i64) % mod_val as i64;
                    } else {
                        dp[i][j] = dp[i - 1][j];
                    }
                }
            }
        }
        let mut sum = 0;
        for &i in dp[n].iter() {
            sum = (sum + i as i64) % mod_val as i64;
        }
        sum = (sum * 2) % mod_val as i64;
        let mut total = 1;
        let mut a = 2;
        let mut n = n as i32;
        while n > 0 {
            if n & 1 == 1 {
                total = (total as i64 * a as i64) % mod_val as i64;
            }
            a = (a as i64 * a as i64) % mod_val as i64;
            n >>= 1;
        }
        ((total as i64 - sum + mod_val as i64) % mod_val as i64) as i32

    }
}
