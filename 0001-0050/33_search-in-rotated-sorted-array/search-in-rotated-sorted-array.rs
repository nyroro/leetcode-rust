
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        while left <= right {
            let mid = (left + right) / 2;
            let mid_val = nums[mid];
            
            if mid_val == target {
                return mid as i32;
            }
            
            if mid_val >= nums[left] {
                if target >= nums[left] && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if target > mid_val && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        
        -1

    }
}
