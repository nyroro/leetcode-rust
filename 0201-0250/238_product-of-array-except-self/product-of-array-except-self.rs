
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut result = vec![0; n];
        
        for i in 1..n {
            left[i] = left[i-1] * nums[i-1];
        }
        
        for i in (0..n-1).rev() {
            right[i] = right[i+1] * nums[i+1];
        }
        
        for i in 0..n {
            result[i] = left[i] * right[i];
        }
        
        result

    }
}
