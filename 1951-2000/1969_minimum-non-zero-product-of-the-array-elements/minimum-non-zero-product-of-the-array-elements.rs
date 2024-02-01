
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let modulo = 1_000_000_007;
        let max_num = (1 << p) - 1;
        let max_product = (max_num - 1) % modulo;
        let non_zero_product = Self::fast_pow(max_product, max_num / 2);
        (non_zero_product * max_num % modulo) as i32

    }
    
    fn fast_pow(base: i64, exponent: i64) -> i64 {
        let modulo = 1_000_000_007;
        let mut result = 1;
        let mut base = base % modulo;
        let mut exponent = exponent;
        
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % modulo;
            }
            base = (base * base) % modulo;
            exponent /= 2;
        }
        
        result

    }
}
