
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let bits = (n as f64).log2().ceil() as u32;
        let mask = (1 << bits) - 1;
        let complement = n ^ mask;
        complement

    }
}
