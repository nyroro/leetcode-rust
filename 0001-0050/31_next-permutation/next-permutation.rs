
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len().wrapping_sub(2) as i32;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        
        if i >= 0 {
            let mut j = nums.len().wrapping_sub(1) as i32;
            while j > i && nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }
        
        let mut left = (i + 1) as usize;
        let mut right = nums.len().wrapping_sub(1) as usize;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
