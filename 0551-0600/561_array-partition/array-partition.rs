
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序

        let mut sum = 0;
        for i in (0..nums.len()).step_by(2) {
            sum += nums[i]; // 取每对中较小的数相加

        }
        sum

    }
}
