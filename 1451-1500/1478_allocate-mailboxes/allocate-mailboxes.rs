
impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let n = houses.len();
        let mut houses = houses;
        houses.sort();

        let mut cost = vec![vec![0; n]; n];
        for i in 0..n {
            for j in i..n {
                let median = houses[(i + j) / 2];
                for l in i..=j {
                    cost[i][j] += (houses[l] - median).abs();
                }
            }
        }

        let mut dp = vec![vec![std::i32::MAX; k as usize + 1]; n];
        for i in 0..n {
            dp[i][1] = cost[0][i];
        }

        for i in 1..n {
            for j in 2..=k as usize {
                for l in 0..i {
                    if dp[l][j - 1] != std::i32::MAX {
                        dp[i][j] = dp[i][j].min(dp[l][j - 1] + cost[l + 1][i]);
                    }
                }
            }
        }

        dp[n - 1][k as usize]
    }
}
