
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = k as usize;
        let mut right = k as usize;
        let mut min_val = nums[k as usize];
        let mut max_score = nums[k as usize];

        while left > 0 || right < nums.len() - 1 {
            if (left > 0 && right < nums.len() - 1 && nums[left - 1] > nums[right + 1]) || right == nums.len() - 1 {
                left -= 1;
                min_val = min_val.min(nums[left]);
            } else {
                right += 1;
                min_val = min_val.min(nums[right]);
            }
            max_score = max_score.max(min_val * (right - left + 1) as i32);
        }

        max_score

    }
}
