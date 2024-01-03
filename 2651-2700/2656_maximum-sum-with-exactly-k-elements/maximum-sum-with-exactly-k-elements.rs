
impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut score = 0;
        for _ in 0..k {
            let max_index = nums.iter().enumerate().max_by_key(|(_, &v)| v).map(|(i, _)| i).unwrap();
            let max_value = nums[max_index];
            score += max_value;
            nums[max_index] += 1;
        }
        score

    }
}
