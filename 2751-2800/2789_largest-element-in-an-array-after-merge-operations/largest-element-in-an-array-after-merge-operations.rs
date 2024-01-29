
impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        fn max_array_value_recursive(nums: &[i32], x: usize) -> i64 {
            let mut ans = nums[x] as i64;
            for i in x..nums.len() - 1 {
                if nums[i] <= nums[i + 1] {
                    ans += nums[i + 1] as i64;
                } else {
                    let right = max_array_value_recursive(&nums, i + 1);
                    if right >= nums[i] as i64 {
                        ans += right;
                    }
                    break;
                }
            }
            ans

        }

        max_array_value_recursive(&nums, 0)
    }
}
