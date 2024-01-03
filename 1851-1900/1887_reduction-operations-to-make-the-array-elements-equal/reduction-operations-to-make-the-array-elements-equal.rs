
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable(); // 对数组进行排序

        let mut operations = 0; // 操作次数

        let mut largest = nums[nums.len() - 1]; // 当前最大值


        for i in (0..nums.len() - 1).rev() {
            if nums[i] < largest {
                largest = nums[i]; // 更新当前最大值

                operations += (nums.len() - i - 1) as i32; // 增加操作次数

            }
        }
        operations

    }
}
