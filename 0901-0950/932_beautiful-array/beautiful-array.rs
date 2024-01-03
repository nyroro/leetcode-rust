
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![1];
        }
        
        let mut left = Self::beautiful_array((n + 1) / 2);
        let mut right = Self::beautiful_array(n / 2);
        
        for i in 0..left.len() {
            left[i] = left[i] * 2 - 1;
        }
        
        for i in 0..right.len() {
            right[i] = right[i] * 2;
        }
        
        left.extend(right);
        left

    }
}
