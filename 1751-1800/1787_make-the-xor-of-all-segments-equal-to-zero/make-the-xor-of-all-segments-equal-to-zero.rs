


impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        const LIMIT: usize = 1 << 10;
        let k = k as usize;
        let mut mrr = vec![vec![0; LIMIT]; k];
        
        for (i, &x) in nums.iter().enumerate() {
            mrr[i % k][x as usize] += 1;
        }

        let mut dp = vec![-2000; LIMIT];
        dp[0] = 0;

        for row in mrr {
            let maxprev = *dp.iter().max().unwrap();
            let mut new_dp = vec![maxprev; LIMIT];
            for (i, &cnt) in row.iter().enumerate() {
                if cnt > 0 {
                    for (j, &prev) in dp.iter().enumerate() {
                        new_dp[i ^ j] = new_dp[i ^ j].max(prev + cnt);
                    }
                }
            }
            dp = new_dp;
        }

        (nums.len() as i32) - dp[0]
    }
}
