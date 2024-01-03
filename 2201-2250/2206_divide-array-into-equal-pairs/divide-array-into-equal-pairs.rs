
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        // 将数组进行排序

        let mut sorted_nums = nums;
        sorted_nums.sort();

        // 逐对检查数组中的元素

        for i in (0..sorted_nums.len()).step_by(2) {
            if sorted_nums[i] != sorted_nums[i + 1] {
                return false;
            }
        }

        // 如果所有的元素都能成对出现且相等，则返回 true

        true

    }
}
