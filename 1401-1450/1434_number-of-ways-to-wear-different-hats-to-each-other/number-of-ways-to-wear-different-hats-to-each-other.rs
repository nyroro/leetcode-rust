
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = hats.len();
        let m = 40;
        let mut dp = vec![0; 1 << n];
        dp[0] = 1;
        let mut people = vec![vec![]; m];
        
        for i in 0..n {
            for &hat in &hats[i] {
                people[(hat - 1) as usize].push(i);
            }
        }
        
        for i in 0..m {
            for j in (0..1 << n).rev() {
                for &person in &people[i] {
                    if (j >> person) & 1 == 1 {
                        continue;
                    }
                    dp[j | (1 << person)] = (dp[j | (1 << person)] + dp[j]) % MOD;
                }
            }
        }
        
        dp[(1 << n) - 1]
    }
}
