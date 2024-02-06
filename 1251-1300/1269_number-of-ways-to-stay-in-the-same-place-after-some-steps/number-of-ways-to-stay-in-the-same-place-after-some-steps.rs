
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        // Define the modulo value

        let modulo: i64 = 1_000_000_007;

        // Calculate the maximum position the pointer can reach

        let max_position = std::cmp::min(steps / 2, arr_len - 1);

        // Create a vector to store the number of ways for each position

        let mut dp = vec![0; (max_position + 1) as usize];
        dp[0] = 1;

        // Iterate for each step

        for _ in 0..steps {
            let mut new_dp = vec![0; (max_position + 1) as usize];
            for j in 0..=max_position {
                new_dp[j as usize] = dp[j as usize];
                if j > 0 {
                    new_dp[j as usize] = (new_dp[j as usize] + dp[(j - 1) as usize]) % modulo;
                }
                if j < max_position {
                    new_dp[j as usize] = (new_dp[j as usize] + dp[(j + 1) as usize]) % modulo;
                }
            }
            dp = new_dp;
        }

        // Return the number of ways for the 0th index

        dp[0] as i32

    }
}
