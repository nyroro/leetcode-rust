


const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();

        for query in queries {
            let n = query[0] as usize;
            let k = query[1] as i64;

            let mut ways = 1;
            let mut i = 2;
            let mut num = k;

            while i * i <= num {
                let mut count = 0;
                while num % i == 0 {
                    count += 1;
                    num /= i;
                }
                ways = (ways * Solution::comb(count + n - 1, count)) % MOD;
                i += 1;
            }

            if num > 1 {
                ways = (ways * n as i64) % MOD;
            }

            result.push(ways as i32);
        }

        result

    }

    fn comb(mut n: usize, mut k: usize) -> i64 {
        if k > n - k {
            k = n - k;
        }
        let mut res = 1;
        for i in 0..k {
            res = (res * (n - i) as i64) % MOD;
            res = (res * Solution::mod_inv(i as i64 + 1)) % MOD;
        }
        res

    }

    fn mod_inv(mut x: i64) -> i64 {
        let mut res = 1;
        let mut p = MOD - 2;
        while p > 0 {
            if p % 2 == 1 {
                res = (res * x) % MOD;
            }
            x = (x * x) % MOD;
            p /= 2;
        }
        res

    }
}
