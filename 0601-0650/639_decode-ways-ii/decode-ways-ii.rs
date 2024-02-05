
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const M: i64 = 1_000_000_007;
        let mut a: i64 = 0;
        let mut b: i64 = 1;

        for (prev_ch, ch) in std::iter::once('0').chain(s.chars()).zip(s.chars()) {
            let (mut p, mut q) = (0, 0);

            // Number of ways considering `prev_ch + ch` as a single number.
            match (prev_ch, ch) {
                ('1', '*') => p = 9,
                ('2', '*') => p = 6,
                ('*', '*') => p = 15,
                ('1', _) => p = 1,
                ('2', x) => p = if x <= '6' { 1 } else { 0 },
                ('*', x) => p = if x <= '6' { 2 } else { 1 },
                _ => {}
            }

            // Number of ways considering only `ch` as a single number.
            match ch {
                '*' => q = 9,
                '0' => q = 0,
                _ => q = 1,
            }

            let temp_a = a;
            a = b;
            b = (temp_a * p + b * q) % M;
        }

        b as i32

    }
}
