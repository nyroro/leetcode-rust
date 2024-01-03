
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对数组进行升序排序

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut min_max_sum = 0;
        while left < right {
            let pair_sum = nums[left] + nums[right]; // 计算当前配对的和

            min_max_sum = min_max_sum.max(pair_sum); // 更新最大和

            left += 1;
            right -= 1;
        }
        min_max_sum

    }
}
