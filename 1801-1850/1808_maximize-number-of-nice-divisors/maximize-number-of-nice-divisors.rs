
impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        let modulo = 1_000_000_007;
        if prime_factors <= 3 {
            return prime_factors;
        }
        let mut power = prime_factors / 3;
        let mut rem = prime_factors % 3;
        if rem == 1 {
            power -= 1;
            rem = 4;
        } else if rem == 0 {
            rem = 1;
        }
        let mut result = 1_i64;
        let mut base = 3_i64;
        while power > 0 {
            if power % 2 == 1 {
                result = (result * base) % modulo;
            }
            base = (base * base) % modulo;
            power /= 2;
        }
        (result * rem as i64 % modulo) as i32

    }
}
