
impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
        let s: Vec<u8> = s.bytes().map(|c| c - b'a').collect();
        let mut ans: i64 = 0;
        let MOD: i64 = 1_000_000_007;
        let mut cnt: Vec<i32> = vec![0; 26];
        let mut t: i64 = 1;
        let mut d: i64 = 1;
        let mut step: i64 = 1;
        cnt[s[s.len() - 1] as usize] = 1;

        for i in (0..s.len() - 1).rev() {
            d = (d * (cnt[s[i] as usize] as i64 + 1)) % MOD;
            t = (t * step) % MOD;
            let g = Solution::gcd(d, t);
            d = (d * Solution::mod_inv(g, MOD)) % MOD;
            t = (t * Solution::mod_inv(g, MOD)) % MOD;
            let mut temp: i64 = 0;
            for j in 0..(s[i] as usize) {
                temp = (temp + cnt[j] as i64) % MOD;
            }
            ans = (ans + (t * temp % MOD) * Solution::mod_inv(d, MOD) % MOD) % MOD;
            cnt[s[i] as usize] += 1;
            step += 1;
        }

        ans as i32

    }

    fn gcd(mut x: i64, mut y: i64) -> i64 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x

    }

    fn mod_inv(mut a: i64, mut m: i64) -> i64 {
        let mut m0 = m;
        let (mut x0, mut x1) = (0, 1);
        if m == 1 {
            return 0;
        }
        while a > 1 {
            let q = a / m;
            let mut t = m;
            m = a % m;
            a = t;
            t = x0;
            x0 = x1 - q * x0;
            x1 = t;
        }
        if x1 < 0 {
            x1 += m0;
        }
        x1

    }
}
