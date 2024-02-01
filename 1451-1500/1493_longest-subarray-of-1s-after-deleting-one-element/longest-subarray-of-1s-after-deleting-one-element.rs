
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut max_len = 0;
        let mut count_zero = 0;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                count_zero += 1;
            }

            while count_zero > 1 {
                if nums[left] == 0 {
                    count_zero -= 1;
                }
                left += 1;
            }

            max_len = max_len.max(right - left);
        }

        if max_len == nums.len() {
            (max_len - 1) as i32

        } else {
            max_len as i32

        }
    }
}
