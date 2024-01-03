
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()]; // 创建一个相同长度的数组ans，并初始化为0

        for i in 0..nums.len() {
            ans[i] = nums[nums[i] as usize]; // 根据题目要求，将ans[i]赋值为nums[nums[i]]
        }
        ans // 返回结果数组ans

    }
}
