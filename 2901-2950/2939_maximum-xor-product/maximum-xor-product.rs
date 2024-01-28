


impl Solution {
    pub fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
        let MOD: i64 = 1_000_000_007;
        let mut a = a;
        let mut b = b;
        for i in (0..n).rev() {
            let mask = 1 << i;
            if (a & mask) != 0 && (b & mask) != 0 {
                continue;
            } else if (a & mask) != 0 {
                if a > b {
                    a ^= mask;
                    b |= mask;
                }
            } else if (b & mask) != 0 {
                if a < b {
                    a |= mask;
                    b ^= mask;
                }
            } else {
                a |= mask;
                b |= mask;
            }
        }
        a %= MOD;
        b %= MOD;
        ((a * b) % MOD) as i32

    }
}
