
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max_length = 0;
        let mut window = std::collections::HashMap::new();

        while right < nums.len() {
            *window.entry(nums[right]).or_insert(0) += 1;

            while *window.keys().max().unwrap() - *window.keys().min().unwrap() > limit {
                *window.get_mut(&nums[left]).unwrap() -= 1;
                if window[&nums[left]] == 0 {
                    window.remove(&nums[left]);
                }
                left += 1;
            }

            max_length = max_length.max(right - left + 1);
            right += 1;
        }

        max_length as i32

    }
}
