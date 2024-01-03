
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        let n = bits.len();
        let mut result = false;
        
        while i < n {
            if bits[i] == 0 {
                result = true;
                i += 1;
            } else {
                result = false;
                i += 2;
            }
        }
        
        result

    }
}
