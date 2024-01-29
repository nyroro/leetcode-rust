


impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn get_candidates(n: i32, p: i32) -> Vec<i32> {
            (1..=n).map(|i| i.pow(p as u32)).take_while(|&num| num <= n).collect()
        }

        let candidates = get_candidates(n, x);
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;

        for &c in &candidates {
            for i in (c..=n).rev() {
                dp[i as usize] = (dp[i as usize] + dp[(i - c) as usize]) % MOD;
            }
        }

        dp[n as usize]
    }
}
