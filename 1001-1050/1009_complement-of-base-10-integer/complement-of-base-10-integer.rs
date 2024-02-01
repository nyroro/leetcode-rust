
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        
        let bits = (n as f64).log2().floor() as u32 + 1;
        let mask = (1 << bits) - 1;
        let complement = n ^ mask;
        complement

    }
}
