
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut total_sum = nums[0];
        let mut current_max = nums[0];
        let mut current_min = nums[0];
        
        for i in 1..n {
            total_sum += nums[i];
            current_max = nums[i].max(current_max + nums[i]);
            max_sum = max_sum.max(current_max);
            current_min = nums[i].min(current_min + nums[i]);
            min_sum = min_sum.min(current_min);
        }
        
        if max_sum < 0 {
            max_sum

        } else {
            max_sum.max(total_sum - min_sum)
        }
    }
}
