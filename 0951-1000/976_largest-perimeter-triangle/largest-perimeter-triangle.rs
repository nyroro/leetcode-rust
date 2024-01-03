
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a)); // 对数组进行降序排序


        for i in 0..nums.len() - 2 {
            let a = nums[i];
            let b = nums[i + 1];
            let c = nums[i + 2];

            if a < b + c {
                return a + b + c; // 返回最大周长

            }
        }

        0 // 如果无法组成非零面积的三角形，则返回0

    }
}
