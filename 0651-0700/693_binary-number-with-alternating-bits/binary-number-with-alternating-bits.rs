
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut num = n;
        let mut prev_bit = num & 1;
        num >>= 1;
        
        while num > 0 {
            let curr_bit = num & 1;
            if curr_bit == prev_bit {
                return false;
            }
            prev_bit = curr_bit;
            num >>= 1;
        }
        
        true

    }
}
