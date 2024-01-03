
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        Self::helper(n, 0)
    }
    
    fn helper(n: i32, x: i32) -> bool {
        if n == 0 {
            return true;
        }
        if n < 0 || x >= 20 {
            return false;
        }
        Self::helper(n, x + 1) || Self::helper(n - 3_i32.pow(x as u32), x + 1)
    }
}
