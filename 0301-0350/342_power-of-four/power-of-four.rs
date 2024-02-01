
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        
        if n & (n - 1) != 0 {
            return false;
        }
        
        if n & 0x55555555 == 0 {
            return true;
        }
        
        false

    }
}
