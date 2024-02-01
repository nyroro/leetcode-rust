
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 2;
        while i >= 0 && nums[i] >= nums[i + 1] {
            i -= 1;
        }
        
        if i >= 0 {
            let mut j = nums.len() - 1;
            while j > i && nums[j] <= nums[i] {
                j -= 1;
            }
            nums.swap(i, j);
        }
        
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
