
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        while left < right {
            let mid = left + (right - left) / 2;
            
            if mid % 2 == 1 {
                if nums[mid] == nums[mid - 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if nums[mid] == nums[mid + 1] {
                    left = mid + 2;
                } else {
                    right = mid;
                }
            }
        }
        
        nums[left]
    }
}
