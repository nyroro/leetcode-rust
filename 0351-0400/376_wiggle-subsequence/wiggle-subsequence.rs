
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        
        let mut up = vec![1; n];
        let mut down = vec![1; n];
        
        for i in 1..n {
            if nums[i] > nums[i-1] {
                up[i] = down[i-1] + 1;
                down[i] = down[i-1];
            } else if nums[i] < nums[i-1] {
                down[i] = up[i-1] + 1;
                up[i] = up[i-1];
            } else {
                up[i] = up[i-1];
                down[i] = down[i-1];
            }
        }
        
        return std::cmp::max(up[n-1], down[n-1]) as i32;
    }
}
