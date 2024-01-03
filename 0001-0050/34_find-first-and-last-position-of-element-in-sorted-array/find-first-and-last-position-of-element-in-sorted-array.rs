
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = Self::binary_search(&nums, target, true);
        let end = Self::binary_search(&nums, target, false);
        vec![start, end]
    }
    
    fn binary_search(nums: &Vec<i32>, target: i32, is_start: bool) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                result = mid;
                
                if is_start {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result

    }
}
