
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut prev_max = std::i32::MIN;
        let mut max_idx = -1;
        let mut prev_min = std::i32::MAX;
        let mut min_idx = -1;

        for i in index_difference as usize..nums.len() {
            let diff_index = i - index_difference as usize;
            if nums[diff_index] > prev_max {
                prev_max = nums[diff_index];
                max_idx = diff_index as i32;
            }
            if nums[diff_index] < prev_min {
                prev_min = nums[diff_index];
                min_idx = diff_index as i32;
            }
            if nums[i] - prev_min >= value_difference {
                return vec![min_idx, i as i32];
            }
            if prev_max - nums[i] >= value_difference {
                return vec![max_idx, i as i32];
            }
        }

        vec![-1, -1]
    }
}
