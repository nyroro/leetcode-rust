
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort(); // 对数组进行排序


        let mut result = Vec::new();
        let n = nums.len();
        let mid = (n + 1) / 2; // 计算中间位置


        for i in 0..mid {
            result.push(nums[mid - 1 - i]); // 将后一半的元素逆序插入到前一半的元素中

            if i < n - mid {
                result.push(nums[n - 1 - i]);
            }
        }

        *nums = result; // 将结果赋值给原数组

    }
}
