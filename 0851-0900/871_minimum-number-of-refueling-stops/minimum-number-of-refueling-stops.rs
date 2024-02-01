
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![start_fuel; target as usize + 1];
        dp[0] = start_fuel;
        
        for station in stations {
            for j in (0..=target).rev() {
                if j >= station[0] as usize {
                    dp[j] = dp[j].max(dp[j - station[0] as usize] + station[1]);
                }
            }
        }
        
        for i in 0..=target {
            if dp[i as usize] >= target {
                return i;
            }
        }
        
        -1

    }
}
