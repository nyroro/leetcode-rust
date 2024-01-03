
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut xor = x ^ y;
        let mut distance = 0;
        
        while xor != 0 {
            if xor & 1 == 1 {
                distance += 1;
            }
            xor >>= 1;
        }
        
        distance

    }
}
