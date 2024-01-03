
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum(); // 计算数组的总和

        let mut left_sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            if left_sum == total_sum - left_sum - num {
                return i as i32; // 找到中心索引，返回索引值

            }
            left_sum += num;
        }

        -1 // 未找到中心索引，返回 -1

    }
}
