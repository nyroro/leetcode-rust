
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        // 初始化正整数和负整数的个数

        let mut positive_count = 0;
        let mut negative_count = 0;

        // 遍历数组，统计正整数和负整数的个数

        for &num in nums.iter() {
            if num > 0 {
                positive_count += 1;
            } else if num < 0 {
                negative_count += 1;
            }
        }

        // 返回正整数和负整数个数中的较大值

        if positive_count > negative_count {
            positive_count

        } else {
            negative_count

        }
    }
}
