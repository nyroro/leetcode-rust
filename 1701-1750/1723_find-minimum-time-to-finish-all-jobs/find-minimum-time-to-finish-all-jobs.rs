
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len();
        let k = k as usize;
        let mut dp = vec![vec![0; n]; 1 << n];
        let mut sum = vec![0; 1 << n];

        for i in 1..(1 << n) {
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    sum[i] = sum[i ^ (1 << j)] + jobs[j];
                    break;
                }
            }
        }

        for i in 0..(1 << n) {
            for j in 0..n {
                if (i & (1 << j)) == 0 {
                    dp[i | (1 << j)][0] = sum[i | (1 << j)];
                    for l in 1..k {
                        dp[i | (1 << j)][l] = std::cmp::min(dp[i | (1 << j)][l], std::cmp::max(dp[i][l - 1], sum[i | (1 << j)]));
                    }
                }
            }
        }

        dp[(1 << n) - 1][k - 1]
    }
}
