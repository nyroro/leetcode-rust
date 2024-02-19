


const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn distinct_sequences(n: i32) -> i64 {
        let mut memo = vec![vec![vec![-1; 10001]; 7]; 7];
        Solution::dp(n as usize, 0, 0, &mut memo) as i64

    }

    fn dp(n: usize, prev2: usize, prev1: usize, memo: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        if n == 0 {
            return 1;
        }
        if memo[prev1][prev2][n] != -1 {
            return memo[prev1][prev2][n];
        }
        let mut res = 0;
        for i in 1..=6 {
            if i != prev2 && i != prev1 && (prev1 == 0 || Solution::gcd(i as i32, prev1 as i32) == 1) {
                res += Solution::dp(n - 1, prev1, i, memo) % MOD;
                res %= MOD;
            }
        }
        memo[prev1][prev2][n] = res;
        res

    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a

    }
}
