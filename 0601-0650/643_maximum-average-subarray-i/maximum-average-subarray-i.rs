
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut start = 0;
        let mut end = k - 1;
        let mut sum: i32 = nums.iter().take(k).sum();
        let mut max_sum = sum;

        while end < nums.len() - 1 {
            end += 1;
            sum += nums[end];
            sum -= nums[start];
            start += 1;
            max_sum = max_sum.max(sum);
        }

        max_sum as f64 / k as f64

    }
}
