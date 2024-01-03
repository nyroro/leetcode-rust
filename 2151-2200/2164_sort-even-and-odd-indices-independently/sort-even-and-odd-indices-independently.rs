
impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        // 创建一个新的数组，存储原始数组中奇数索引位置的值

        let mut odd_indices: Vec<i32> = Vec::new();
        for i in (1..nums.len()).step_by(2) {
            odd_indices.push(nums[i]);
        }
        // 对奇数索引位置的值进行非递增排序

        odd_indices.sort_by(|a, b| b.cmp(a));
        // 遍历原始数组，将奇数索引位置的值替换为排序后的奇数索引位置值

        for i in (1..nums.len()).step_by(2) {
            nums[i] = odd_indices[(i - 1) / 2];
        }
        // 创建另一个新数组，存储原始数组中偶数索引位置的值

        let mut even_indices: Vec<i32> = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            even_indices.push(nums[i]);
        }
        // 对偶数索引位置的值进行非递减排序

        even_indices.sort();
        // 遍历原始数组，将偶数索引位置的值替换为排序后的偶数索引位置值

        for i in (0..nums.len()).step_by(2) {
            nums[i] = even_indices[i / 2];
        }
        // 返回重新排列后的数组

        nums

    }
}
