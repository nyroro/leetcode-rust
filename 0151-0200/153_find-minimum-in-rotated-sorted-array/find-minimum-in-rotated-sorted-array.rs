
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid];
            
            if mid_val >= nums[0] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        nums[left]
    }
}
