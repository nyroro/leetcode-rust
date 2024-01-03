
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut rotation = 0;

        for i in 0..n {
            sum += nums[i];
            rotation += i as i32 * nums[i];
        }

        let mut max_value = rotation;

        for i in 1..n {
            rotation = rotation + sum - n as i32 * nums[n - i];
            max_value = max_value.max(rotation);
        }

        max_value

    }
}
