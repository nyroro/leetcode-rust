
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n = nums.len();
        let max_num = *nums.iter().max().unwrap(); // 找到nums数组中的最大值

        let mut xor_sum = 0;
        let mut answer = Vec::new();

        for i in (0..n).rev() {
            xor_sum ^= nums[i]; // 计算nums数组中所有元素的异或结果

        }

        for i in (0..n).rev() {
            let k = xor_sum ^ ((1 << maximum_bit) - 1); // 计算当前位上k的取值

            answer.push(k);
            xor_sum ^= nums[i]; // 移除最后一个元素

        }

        answer

    }
}
