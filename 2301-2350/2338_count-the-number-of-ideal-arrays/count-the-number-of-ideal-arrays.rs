
impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        
        fn _add(x: i64, y: i64) -> i64 { (x % MOD + y % MOD) % MOD }
        fn _mul(x: i64, y: i64) -> i64 { (x % MOD * y % MOD) % MOD }
        fn _pow(x: i64, y: i64) -> i64 {
            if y == 0 { 1 }
            else if y % 2 == 0 {
                let tmp = _pow(x, y / 2);
                _mul(tmp, tmp)
            } else {
                _mul(x, _pow(x, y - 1))
            }
        }
        fn _inv(p: i64) -> i64 { _pow(p, MOD - 2) }
        fn _div(x: i64, y: i64) -> i64 { _mul(x, _inv(y)) }

        let mut dp = vec![vec![0; 16]; (max_value + 2) as usize];
        let mut Arr = vec![0; 16];
        let mut fact = vec![1; (n + 1) as usize];

        for i in (1..=max_value).rev() {
            dp[i as usize][1] += 1;
            for j in ((2 * i)..=max_value).step_by(i as usize) {
                for k in 1..15 {
                    dp[i as usize][k as usize + 1] += dp[j as usize][k as usize];
                }
            }
            for j in 1..=15 {
                Arr[j as usize] += dp[i as usize][j as usize];
            }
        }

        for i in 1..=n as usize {
            fact[i] = _mul(fact[i - 1], i as i64);
        }

        let mut result = 0;
        for i in 1..=std::cmp::min(15, n) as usize {
            let temp = _div(fact[n as usize - 1], _mul(fact[i - 1], fact[n as usize - i]));
            result = _add(result, _mul(temp, Arr[i]));
        }

        result as i32

    }
}
