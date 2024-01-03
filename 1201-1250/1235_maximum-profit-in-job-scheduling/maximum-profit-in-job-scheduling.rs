
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32, i32)> = start_time

            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((&s, &e), &p)| (s, e, p))
            .collect();
        
        jobs.sort_by_key(|&(_, e, _)| e);
        
        let n = jobs.len();
        let mut dp = vec![0; n + 1];
        
        for i in 1..=n {
            let (s, e, p) = jobs[i - 1];
            let mut j = i - 1;
            
            while j > 0 && jobs[j - 1].1 > s {
                j -= 1;
            }
            
            dp[i] = std::cmp::max(dp[i - 1], dp[j] + p);
        }
        
        dp[n]
    }
}
