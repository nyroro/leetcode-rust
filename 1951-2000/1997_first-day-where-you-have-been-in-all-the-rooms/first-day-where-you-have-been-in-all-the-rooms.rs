
// Define the Solution struct



impl Solution {
    // Define the function to solve the problem

    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i64 {
        let modulo: i64 = 1_000_000_007;
        let n = next_visit.len();
        let mut dp = vec![0; n];
        
        for e in 1..n {
            dp[e] = (dp[e - 1] + 1) % modulo;
            dp[e] = (2 * dp[e] - dp[next_visit[e - 1] as usize] + modulo) % modulo;
        }
        
        dp[n - 1]
    }
}
