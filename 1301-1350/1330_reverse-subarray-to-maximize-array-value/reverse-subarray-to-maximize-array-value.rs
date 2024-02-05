
impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut min_val = i32::MAX;
        let mut max_val = i32::MIN;
        let mut total_value = 0;
        let mut change = 0;

        for i in 0..nums.len() - 1 {
            let a = nums[i];
            let b = nums[i + 1];
            max_val = max_val.max(a.min(b));
            min_val = min_val.min(a.max(b));
            let delta = (a - b).abs();
            total_value += delta;
            let boundary_value = (nums[0] - b).abs().max((nums[nums.len() - 1] - a).abs()) - delta;
            change = change.max(boundary_value).max(2 * (max_val - min_val));
        }

        total_value + change

    }
}
