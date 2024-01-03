
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result = 0;
        let mut n = x;
        for _ in 0..32 {
            result = (result << 1) | (n & 1);
            n >>= 1;
        }
        result

    }
}
