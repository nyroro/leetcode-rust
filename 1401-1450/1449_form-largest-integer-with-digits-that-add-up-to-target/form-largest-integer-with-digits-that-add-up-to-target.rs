
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let mut dp = vec![std::i32::MIN; (target + 1) as usize];
        dp[0] = 0;

        for i in 1..=target {
            for j in 0..9 {
                if i >= cost[j] && dp[(i - cost[j]) as usize] > std::i32::MIN {
                    dp[i as usize] = dp[i as usize].max(dp[(i - cost[j]) as usize] + 1);
                }
            }
        }

        if dp[target as usize] <= std::i32::MIN {
            return "0".to_string();
        }

        let mut result = String::new();
        let mut remaining = target;

        for i in (1..=9).rev() {
            while remaining >= cost[i - 1] && dp[remaining as usize] == dp[(remaining - cost[i - 1]) as usize] + 1 {
                result.push_str(&i.to_string());
                remaining -= cost[i - 1];
            }
        }

        result

    }
}
