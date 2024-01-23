
impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; (n + 1) as usize];
        
        let mut sorted_rides = rides.clone();
        sorted_rides.sort_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
        
        let mut now = 0;
        for r in sorted_rides {
            while now < r[1] {
                dp[(now + 1) as usize] = dp[now as usize].max(dp[(now + 1) as usize]);
                now += 1;
            }
            dp[r[1] as usize] = dp[r[1] as usize].max(dp[r[0] as usize] + (r[1] - r[0] + r[2]) as i64);
        }
        while now < n {
            dp[(now + 1) as usize] = dp[now as usize].max(dp[(now + 1) as usize]);
            now += 1;
        }
        dp[n as usize] as i64

    }
}
