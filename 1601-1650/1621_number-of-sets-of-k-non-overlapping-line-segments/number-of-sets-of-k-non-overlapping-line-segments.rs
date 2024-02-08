
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Define a function to calculate the power with modulo

        fn power(a: i64, b: i64, modulo: i64) -> i64 {
            let mut res = 1;
            let mut a = a;
            let mut b = b;
            while b > 0 {
                if b & 1 == 1 {
                    res = (res * a) % modulo;
                }
                a = (a * a) % modulo;
                b /= 2;
            }
            res

        }

        // Calculate the total number of sets

        let mut curr: i64 = 1;
        let n = n as i64;
        let k = k as i64;
        let N = n + k - 1;
        let r = 2 * k;

        for i in 1..=r {
            curr = (curr * (N - i + 1)) % MOD;
            curr = (curr * power(i, MOD - 2, MOD)) % MOD;
        }

        curr as i32

    }
}
