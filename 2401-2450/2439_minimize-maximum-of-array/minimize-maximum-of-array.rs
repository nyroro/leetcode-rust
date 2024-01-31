
impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut max_avg: f64 = 0.0;
        let mut sum: i64 = 0;

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            let avg = sum as f64 / (i as i64 + 1) as f64;
            max_avg = max_avg.max(avg);
        }

        max_avg.ceil() as i32

    }
}
