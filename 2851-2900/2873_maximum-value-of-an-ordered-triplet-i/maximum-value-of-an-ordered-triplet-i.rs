
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_value = 0;
        let n = nums.len();

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let value = (nums[i] - nums[j]) as i64 * nums[k] as i64;
                    max_value = max_value.max(value);
                }
            }
        }

        max_value.max(0)
    }
}
